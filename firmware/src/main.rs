#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::pac;

// ==========================================
// KONFIGURACJA TESTU
// ==========================================

const ENABLE_MOTOR_TEST: bool = true;

const STOP: u16 = 0;
const TEST_START_THROTTLE: u16 = 120;
const TEST_MAX_THROTTLE: u16 = 400;

const INIT_FRAMES: u32 = 5000; // 10 sekund
const RAMP_FRAMES: u32 = 2500; // 5 seconds at 500 Hz
const TEST_FRAMES: u32 = 1500; // 3 seconds at the final throttle
const PAUSE_FRAMES: u32 = 1500; // 3 seconds of STOP between motors
const MOTOR_COUNT: usize = 4;

// STM32F411 HSI 16 MHz
const CPU_HZ: u32 = 16_000_000;

// DShot150
const BIT_TICKS: u32 = 107;
const HIGH_0: u16 = 40;
const HIGH_1: u16 = 80;

// ==========================================
// ADRESY REJESTROW
// ==========================================

const RCC: u32 = 0x4002_3800;
const GPIOA: u32 = 0x4002_0000;
const GPIOB: u32 = 0x4002_0400;
const GPIOC: u32 = 0x4002_0800;
const TIM3: u32 = 0x4000_0400;
const DMA1: u32 = 0x4002_6000;

// DMA1 Stream2 Channel5
const DMA_CR: u32 = DMA1 + 0x40;
const DMA_NDTR: u32 = DMA1 + 0x44;
const DMA_PAR: u32 = DMA1 + 0x48;
const DMA_M0AR: u32 = DMA1 + 0x4C;
const DMA_FCR: u32 = DMA1 + 0x54;

// DMA Stream2 flags
const DMA_TC: u32 = 1 << 21;

const DMA_ERRORS: u32 = (1 << 19) | (1 << 18) | (1 << 16);

const DMA_CLEAR: u32 = (1 << 21) | (1 << 20) | (1 << 19) | (1 << 18) | (1 << 16);

// ==========================================
// ODCZYT I ZAPIS REJESTROW
// ==========================================

#[inline(always)]
fn read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[inline(always)]
fn write(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}

#[inline(always)]
fn modify(addr: u32, clear: u32, set: u32) {
    write(addr, (read(addr) & !clear) | set);
}

// ==========================================
// OPOZNIENIA
// ==========================================

fn cycles() -> u32 {
    cortex_m::peripheral::DWT::cycle_count()
}

fn delay_us(us: u32) {
    let start = cycles();
    let ticks = us * (CPU_HZ / 1_000_000);

    while cycles().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
}

// ==========================================
// DIODA PC13
// ==========================================

// LOW = LED ON
// HIGH = LED OFF

fn led(on: bool) {
    if on {
        write(GPIOC + 0x18, 1 << 29);
    } else {
        write(GPIOC + 0x18, 1 << 13);
    }
}

// ==========================================
// ZEGAR
// ==========================================

fn init_clock() {
    // Wlacz HSI
    modify(RCC + 0x00, 0, 1);

    while read(RCC + 0x00) & (1 << 1) == 0 {}

    // SYSCLK = HSI
    // AHB, APB1, APB2 = /1
    modify(RCC + 0x08, 0x0000_FCF3, 0);

    while read(RCC + 0x08) & (3 << 2) != 0 {}

    // GPIOA, GPIOB, GPIOC, DMA1
    modify(RCC + 0x30, 0, (1 << 0) | (1 << 1) | (1 << 2) | (1 << 21));

    // TIM3
    modify(RCC + 0x40, 0, 1 << 1);

    let _ = read(RCC + 0x40);
}

// ==========================================
// GPIO
// ==========================================

fn init_gpio() {
    // PC13 jako wyjscie
    modify(GPIOC + 0x00, 3 << 26, 1 << 26);

    // PC13 push-pull
    modify(GPIOC + 0x04, 1 << 13, 0);
    led(false);

    // ESC 1..4: PA6/TIM3_CH1, PA7/TIM3_CH2,
    // PB0/TIM3_CH3, PB1/TIM3_CH4. All use AF2.
    for (port, pin) in [(GPIOA, 6u32), (GPIOA, 7), (GPIOB, 0), (GPIOB, 1)] {
        modify(port + 0x00, 3 << (2 * pin), 2 << (2 * pin));
        modify(port + 0x04, 1 << pin, 0);
        modify(port + 0x08, 3 << (2 * pin), 2 << (2 * pin));
        modify(port + 0x0C, 3 << (2 * pin), 0);
        modify(port + 0x20, 0xF << (4 * pin), 2 << (4 * pin));
    }
}

// ==========================================
// TIMER TIM3
// ==========================================

fn init_timer() {
    // Timer OFF
    write(TIM3 + 0x00, 0);

    // DMA requests OFF
    write(TIM3 + 0x0C, 0);

    // Prescaler = 0
    write(TIM3 + 0x28, 0);

    // 107 taktow na bit
    write(TIM3 + 0x2C, BIT_TICKS - 1);

    // Start all four outputs LOW.
    for channel in 0..MOTOR_COUNT {
        write(TIM3 + 0x34 + 4 * channel as u32, 0);
    }

    // PWM mode 1 with preload on channels 1..4.
    let pwm_pair = (6 << 4) | (1 << 3) | (6 << 12) | (1 << 11);
    write(TIM3 + 0x18, pwm_pair);
    write(TIM3 + 0x1C, pwm_pair);

    // Enable all four channels, active high.
    write(TIM3 + 0x20, 0x1111);

    // DMA writes directly to a selected CCR; timer DMA burst is unused.
    write(TIM3 + 0x48, 0);

    // Auto-reload preload
    write(TIM3 + 0x00, 1 << 7);

    // Zaladuj rejestry
    write(TIM3 + 0x14, 1);

    // Wyczysc flagi
    write(TIM3 + 0x10, 0);
}

// ==========================================
// PAKIET DSHOT
// ==========================================

fn make_packet(throttle: u16) -> u16 {
    let payload = (throttle & 0x07FF) << 1;

    let mut data = payload;
    let mut crc = 0u16;

    for _ in 0..3 {
        crc ^= data & 0x0F;
        data >>= 4;
    }

    (payload << 4) | (crc & 0x0F)
}

fn pulse_width(packet: u16, bit: usize) -> u16 {
    if bit >= 16 {
        return 0;
    }

    if ((packet >> (15 - bit)) & 1) != 0 {
        HIGH_1
    } else {
        HIGH_0
    }
}

// ==========================================
// DMA STOP
// ==========================================

fn disable_dma() -> bool {
    modify(DMA_CR, 1, 0);

    let start = cycles();

    while read(DMA_CR) & 1 != 0 {
        if cycles().wrapping_sub(start) > CPU_HZ / 1000 {
            return false;
        }
    }

    true
}

// ==========================================
// AWARYJNE ZATRZYMANIE SYGNALU
// ==========================================

fn emergency_stop() {
    // DMA request OFF
    modify(TIM3 + 0x0C, 1 << 8, 0);

    // TIM3 OFF
    modify(TIM3 + 0x00, 1, 0);

    // DMA OFF
    modify(DMA_CR, 1, 0);

    // Set output latches LOW before switching all signal pins to GPIO.
    write(GPIOA + 0x18, (1 << 22) | (1 << 23));
    write(GPIOB + 0x18, (1 << 16) | (1 << 17));
    modify(GPIOA + 0x00, (3 << 12) | (3 << 14), (1 << 12) | (1 << 14));
    modify(GPIOB + 0x00, 3 | (3 << 2), 1 | (1 << 2));
}

fn error_loop() -> ! {
    emergency_stop();

    loop {
        led(true);
        delay_us(100_000);

        led(false);
        delay_us(100_000);
    }
}

// ==========================================
// TRANSMISJA DSHOT150
// ==========================================

fn send_dshot(motor: usize, throttle: u16) -> bool {
    if motor >= MOTOR_COUNT {
        emergency_stop();
        return false;
    }

    let packet = make_packet(throttle);
    let ccr = TIM3 + 0x34 + 4 * motor as u32;

    // Start with one LOW period and bit 0 in the selected CCR preload.
    // Each hardware update activates the preload before DMA writes
    // the next value: bits 1..15 followed by three LOW periods.

    let mut buffer = [0u16; 18];

    for (bit, duty) in buffer.iter_mut().enumerate() {
        *duty = pulse_width(packet, bit + 1);
    }

    // Timer i DMA request OFF
    modify(TIM3 + 0x00, 1, 0);
    modify(TIM3 + 0x0C, 1 << 8, 0);

    if !disable_dma() {
        return false;
    }

    // Wyczysc flagi Stream2
    write(DMA1 + 0x08, DMA_CLEAR);

    // CNT = 0
    write(TIM3 + 0x24, 0);

    // Keep the active compare value at zero throughout DMA setup.
    for channel in 0..MOTOR_COUNT {
        write(TIM3 + 0x34 + 4 * channel as u32, 0);
    }

    // Wymus update
    write(TIM3 + 0x14, 1);

    // Bit 0 becomes active only on the first timer overflow.
    write(ccr, pulse_width(packet, 0) as u32);

    // Flagi timera
    write(TIM3 + 0x10, 0);

    // Update requests feed the selected channel's CCR directly.
    write(DMA_PAR, ccr);

    // DMA memory = buffer
    write(DMA_M0AR, buffer.as_ptr() as u32);

    // Ilosc transferow
    write(DMA_NDTR, buffer.len() as u32);

    // Direct mode
    write(DMA_FCR, 0);

    // Channel 5
    // High priority
    // Halfword -> Halfword
    // Memory increment
    // Memory -> Peripheral
    write(
        DMA_CR,
        (5 << 25) | (2 << 16) | (1 << 13) | (1 << 11) | (1 << 10) | (1 << 6),
    );

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    // DMA ON
    modify(DMA_CR, 0, 1);

    // TIM3 update DMA request
    modify(TIM3 + 0x0C, 0, 1 << 8);

    // Start timer
    write(TIM3 + 0x24, 0);
    modify(TIM3 + 0x00, 0, 1);

    let start = cycles();

    // Timeout 300 us
    let timeout = CPU_HZ / 1_000_000 * 300;

    loop {
        let status = read(DMA1);

        // Transfer error
        if status & DMA_ERRORS != 0 {
            emergency_stop();
            return false;
        }

        // Transfer complete
        if status & DMA_TC != 0 {
            break;
        }

        if cycles().wrapping_sub(start) > timeout {
            emergency_stop();
            return false;
        }
    }

    // DMA completion means the third trailing zero reached the preload;
    // the active compare value is already zero. Allow two full periods
    // before stopping, measured from completion rather than timer startup.
    delay_us((2 * BIT_TICKS).div_ceil(CPU_HZ / 1_000_000));

    // Wylacz TIM3 DMA request
    modify(TIM3 + 0x0C, 1 << 8, 0);

    // Zatrzymaj timer
    modify(TIM3 + 0x00, 1, 0);

    if !disable_dma() {
        emergency_stop();
        return false;
    }

    // Przygotuj LOW
    for channel in 0..MOTOR_COUNT {
        write(TIM3 + 0x34 + 4 * channel as u32, 0);
    }
    write(TIM3 + 0x14, 1);

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    true
}

// ==========================================
// WYSYLANIE WIELU RAMEK
// ==========================================

fn send_for(motor: usize, throttle: u16, frames: u32, blink: bool) -> bool {
    if motor >= MOTOR_COUNT {
        emergency_stop();
        return false;
    }

    for i in 0..frames {
        let start = cycles();

        if blink && i % 250 == 0 {
            led((i / 250) % 2 == 0);
        }

        // Send separate frames, avoiding timer DMA burst. Each motor still
        // receives one frame per 2 ms; only the selected one gets throttle.
        for channel in 0..MOTOR_COUNT {
            let value = if channel == motor { throttle } else { STOP };
            if !send_dshot(channel, value) {
                return false;
            }
        }

        // 500 ramek na sekunde
        while cycles().wrapping_sub(start) < CPU_HZ / 500 {
            core::hint::spin_loop();
        }
    }

    true
}

// ==========================================
// MAIN
// ==========================================

#[entry]
fn main() -> ! {
    let _dp = pac::Peripherals::take().unwrap();

    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    init_clock();

    // Licznik cykli DWT
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    init_gpio();
    init_timer();

    // Sygnal uruchomienia programu
    led(true);
    delay_us(500_000);

    led(false);
    delay_us(500_000);

    // ======================================
    // ETAP 1 - 10 sekund STOP
    // ======================================

    if !send_for(0, STOP, INIT_FRAMES, true) {
        error_loop();
    }

    // ======================================
    // STAGE 2 - test M1, M2, M3, M4 sequentially
    // ======================================

    if ENABLE_MOTOR_TEST {
        for motor in 0..MOTOR_COUNT {
            led(true);

            // Only the selected motor receives throttle; all others get STOP.
            for frame in 0..RAMP_FRAMES {
                let throttle = TEST_START_THROTTLE
                    + ((TEST_MAX_THROTTLE - TEST_START_THROTTLE) as u32 * frame / (RAMP_FRAMES - 1))
                        as u16;

                if !send_for(motor, throttle, 1, false) {
                    error_loop();
                }
            }

            if !send_for(motor, TEST_MAX_THROTTLE, TEST_FRAMES, false) {
                error_loop();
            }

            if !send_for(0, STOP, PAUSE_FRAMES, true) {
                error_loop();
            }
        }
    }

    // ======================================
    // ETAP 3 - STOP na stale
    // ======================================

    loop {
        if !send_for(0, STOP, 2500, true) {
            error_loop();
        }
    }
}
