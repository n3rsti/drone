# Drone — lista komponentów technicznych

## 1. FC / mikrokontroler

### STM32F411CEU6 / STM32 BlackPill F411

| Parametr | Wartość |
|---|---|
| Układ MCU | STM32F411CEU6 |
| Rdzeń | ARM Cortex-M4 32-bit + FPU |
| Taktowanie maksymalne | 100 MHz |
| Flash | 512 KB |
| SRAM | 128 KB |
| Zasilanie MCU | 1.7–3.6 V |
| Logika I/O | 3.3 V |
| Obudowa samego STM32F411CEU6 | UFQFPN48 |
| Wymiary układu STM32F411CEU6 | 7 × 7 mm |
| Wymiary płytki BlackPill F411 | ok. 20.78 × 52.81 mm |
| Programowanie | SWD |
| USB | USB FS / USB-C lub microUSB, zależnie od wersji BlackPill |
| Dokumentacja STM32F411CEU6 | [STMicroelectronics — STM32F411CE datasheet](https://www.st.com/resource/en/datasheet/stm32f411ce.pdf) |
| Dokumentacja STM32F411 reference manual | [STMicroelectronics — RM0383 STM32F411 reference manual](https://www.st.com/en/microcontrollers-microprocessors/stm32f411ce.html#documentation) |
| Dokumentacja BlackPill | [WeActStudio MiniSTM32F4x1 GitHub](https://github.com/WeActStudio/WeActStudio.MiniSTM32F4x1) |
| Dane płytki BlackPill | [STM32-base — WeAct BlackPill F411](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0.html) |

### Połączenia FC

| Funkcja | Połączenie |
|---|---|
| Zasilanie FC | BEC 5 V -> pin 5V BlackPill |
| Masa | GND BlackPill -> wspólna masa |
| Czujniki I2C | SDA/SCL BlackPill -> SDA/SCL MPU6050, BMP280, QMC5883L |
| Odbiornik ELRS | UART TX/RX BlackPill -> RX/TX odbiornika |
| ESC | 4 piny sygnałowe BlackPill -> S1/S2/S3/S4 ESC |
| Pomiar napięcia baterii | VBAT -> dzielnik napięcia -> ADC BlackPill |
| Programowanie | ST-Link SWDIO/SWCLK/GND/3V3 -> piny SWD BlackPill |

---

## 2. Odbiornik RC

### Flywoo TCXO 2.4G ELRS EL24P

| Parametr | Wartość |
|---|---|
| Typ | Odbiornik RC ExpressLRS |
| Pasmo | 2.4 GHz |
| Zakres częstotliwości | 2400–2500 MHz |
| MCU | ESP8285 |
| Układ RF | SX1280 / SX1281 |
| Protokół komunikacji z FC | CRSF |
| Interfejs do FC | UART |
| Odświeżanie | 25–500 Hz |
| Zasilanie | 5 V |
| Masa | ok. 0.44 g bez anteny |
| Wymiary | 10 × 10 × 6 mm |
| Antena | T-antenna, zależnie od wersji zestawu |
| Dokumentacja / strona producenta | [Flywoo — TCXO 2.4G ELRS EL24E / EL24P](https://flywoo.net/products/0-4g-flywoo-2-4g-expresslrs-elrs-el24e-el24p) |

### Połączenia odbiornika

| Pin odbiornika | Połączenie |
|---|---|
| 5V | BEC 5 V |
| GND | wspólna masa |
| TX | RX UART BlackPill |
| RX | TX UART BlackPill |

---

## 3. Aparatura RC

### Jumper Smart 2.4G Hall ELRS

| Parametr | Wartość |
|---|---|
| Typ | Aparatura RC |
| System radiowy | ExpressLRS 2.4 GHz |
| Gimbale | Hall |
| Firmware | JumperTX |
| Ekran | kolorowy, ok. 2" |
| Moc nadajnika | do 100 mW według danych dystrybucyjnych |
| Wymiary | ok. 152 × 160 × 63 mm według danych dystrybucyjnych |
| Masa | ok. 353 g bez baterii według danych dystrybucyjnych |
| Dokumentacja / strona produktu | [Rotorama — Jumper Smart Hall ELRS](https://www.rotorama.com/product/jumper-smart-2-4g-hall) |
| Dodatkowe dane produktu | [FlyingTech — Jumper Smart 2.4GHz ELRS](https://www.flyingtech.co.uk/product/jumper-smart-2-4ghz-elrs-radio-controller/) |

---

## 4. Bateria

### Tattu LiPo 4S 14.8 V 2300 mAh 75C XT60

| Parametr | Wartość |
|---|---|
| Typ | LiPo |
| Konfiguracja | 4S / 4 cele |
| Napięcie nominalne | 14.8 V |
| Napięcie pełnego pakietu | 16.8 V |
| Pojemność | 2300 mAh |
| C-rating | 75C |
| Energia | ok. 34.04 Wh |
| Złącze główne | XT60 |
| Złącze balansera | JST-XH / JST-XHR |
| Masa | ok. 270 g |
| Wymiary | ok. 105 × 35 × 29 mm / 105 × 33 × 28 mm, zależnie od źródła |
| Dokumentacja / strona producenta | [GensTattu — Tattu 2300mAh 4S 75C XT60](https://www.genstattu.com/tattu-2300mah-4s-75c-lipo-battery-pack-with-xt60-plug/) |
| Dane wymiarowe dystrybutora | [Rotorama — Tattu 2300mAh 4S 75C](https://www.rotorama.com/product/tattu-2300mah-4s-75c) |

### Połączenia baterii

| Wyjście baterii | Połączenie |
|---|---|
| XT60 + | ESC BAT+ oraz wejście BEC + |
| XT60 - | ESC BAT- oraz wejście BEC - |
| Balanser JST-XH / JST-XHR | tylko do ładowania / pomiaru cel |

---

## 5. Programator

### ST-Link V2 SWD STM32 / SWIM STM8

| Parametr | Wartość |
|---|---|
| Typ | Programator/debugger |
| Obsługiwane MCU | STM32 przez SWD, STM8 przez SWIM |
| Interfejs dla STM32F411 | SWD |
| Linie SWD | SWDIO, SWCLK, GND, 3V3 reference, opcjonalnie NRST |
| Dokumentacja ST-Link | [STMicroelectronics — ST-LINK/V2](https://www.st.com/en/development-tools/st-link-v2.html) |
| Oprogramowanie | [STMicroelectronics — STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html) |

### Połączenia ST-Link z BlackPill

| ST-Link | BlackPill / STM32 |
|---|---|
| SWDIO | SWDIO / PA13 |
| SWCLK | SWCLK / PA14 |
| GND | GND |
| 3V3 | 3V3 reference |
| NRST | NRST, opcjonalnie |

---

## 6. Magnetometr

### QMC5883L Magnetometr 3-osiowy, I2C

| Parametr | Wartość |
|---|---|
| Typ | 3-osiowy magnetometr |
| Interfejs | I2C |
| Typowy adres I2C | 0x0D |
| Zasilanie układu | 2.16–3.6 V, zależnie od wersji dokumentacji |
| Obudowa układu | LGA |
| Wymiary samego układu | ok. 3 × 3 × 0.9 mm |
| Wymiary modułu Arduino | zależne od konkretnej płytki |
| Dokumentacja | [QST — QMC5883L datasheet PDF](https://www.qstcorp.com/upload/pdf/202512/13-52-04%20QMC5883L%20Datasheet%20Rev.%20B.pdf) |

### Połączenia QMC5883L

| Pin modułu | Połączenie |
|---|---|
| VCC | 3V3 BlackPill |
| GND | wspólna masa |
| SDA | SDA I2C BlackPill |
| SCL | SCL I2C BlackPill |

---

## 7. Barometr

### BMP280 Czujnik ciśnienia i temperatury

| Parametr | Wartość |
|---|---|
| Typ | Barometr cyfrowy + czujnik temperatury |
| Zakres ciśnienia | 300–1100 hPa |
| Interfejs | I2C / SPI |
| Typowe adresy I2C | 0x76 / 0x77 |
| Zasilanie układu VDD | 1.71–3.6 V |
| Zasilanie I/O VDDIO | 1.2–3.6 V |
| Wymiary samego układu | 2.0 × 2.5 × 0.95 mm |
| Wymiary modułu Arduino | zależne od konkretnej płytki |
| Dokumentacja | [Bosch Sensortec — BMP280 datasheet PDF](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) |

### Połączenia BMP280 w trybie I2C

| Pin modułu | Połączenie |
|---|---|
| VCC / VIN | 3V3 BlackPill |
| GND | wspólna masa |
| SDA | SDA I2C BlackPill |
| SCL | SCL I2C BlackPill |
| CSB | VCC, jeśli wymagane przez moduł dla trybu I2C |
| SDO | GND albo VCC — wybór adresu I2C 0x76 / 0x77 |

---

## 8. IMU

### MPU6050 GY-521 — żyroskop + akcelerometr

| Parametr | Wartość |
|---|---|
| Typ | 6-osiowy IMU |
| Żyroskop | 3 osie |
| Akcelerometr | 3 osie |
| Interfejs | I2C |
| Typowe adresy I2C | 0x68 / 0x69 |
| Zasilanie układu VDD | 2.375–3.46 V |
| Logika I/O | zależna od VLOGIC / modułu |
| Wymiary samego układu | 4 × 4 × 0.9 mm |
| Wymiary modułu GY-521 | zależne od konkretnej płytki |
| Dokumentacja | [TDK/InvenSense — MPU-6000 / MPU-6050 datasheet PDF](https://product.tdk.com/system/files/dam/doc/product/sensor/mortion-inertial/imu/data_sheet/mpu-6000-datasheet1.pdf) |

### Połączenia MPU6050 GY-521

| Pin modułu | Połączenie |
|---|---|
| VCC | 3V3 BlackPill |
| GND | wspólna masa |
| SDA | SDA I2C BlackPill |
| SCL | SCL I2C BlackPill |
| AD0 | GND dla adresu 0x68 albo 3V3 dla 0x69 |
| INT | opcjonalny pin GPIO BlackPill |

---

## 9. Śmigła

### TP 5 × 4.5 3-blade CW black / 5045

| Parametr | Wartość |
|---|---|
| Typ | Śmigło do multirotora |
| Oznaczenie | 5045 |
| Średnica | 5" |
| Skok | 4.5" |
| Liczba łopat | 3 |
| Kierunek z listy | CW |
| Materiał | zależny od producenta |
| Wymiary | 5" średnicy |
| Dokumentacja oficjalna | brak jednoznacznej dokumentacji dla podanego wariantu TP |

### Wymagana konfiguracja dla quada

| Element | Ilość |
|---|---|
| Śmigło 5045 CW | 2 szt. minimum |
| Śmigło 5045 CCW | 2 szt. minimum |

---

## 10. ESC

### Mini 4w1 ESC 45A / 50A burst, 2–6S, DShot150–600

| Parametr | Wartość |
|---|---|
| Typ | 4-in-1 ESC |
| Liczba kanałów | 4 |
| Zasilanie | LiPo 2–6S |
| Prąd ciągły | 45 A |
| Prąd chwilowy | 50 A przez 10 s |
| Protokoły | DShot150 / DShot300 / DShot600, Multishot, prawdopodobnie OneShot / PWM |
| Firmware / konfiguracja | BLHeli / BLHeli_S, do potwierdzenia po modelu |
| Wymiary | 41.0 × 39.5 × 5 mm |
| Rozstaw otworów | 30.5 × 30.5 mm |
| Średnica otworów po gumowych dystansach | ok. 3 mm |
| BEC 5 V | brak potwierdzenia; na zdjęciach brak oznaczonego padu 5V |
| Dokumentacja oficjalna | brak jednoznacznej dokumentacji dla tej płytki |

### Pady / złącza ESC

| Oznaczenie | Funkcja |
|---|---|
| BAT+ / + | wejście zasilania z baterii |
| BAT- / - | masa zasilania z baterii |
| M1 | wyjście 3-fazowe silnika 1 |
| M2 | wyjście 3-fazowe silnika 2 |
| M3 | wyjście 3-fazowe silnika 3 |
| M4 | wyjście 3-fazowe silnika 4 |
| G / GND | masa sygnałowa |
| VT / VBAT | napięcie baterii |
| 1 / S1 | sygnał silnika 1 z FC |
| 2 / S2 | sygnał silnika 2 z FC |
| 3 / S3 | sygnał silnika 3 z FC |
| 4 / S4 | sygnał silnika 4 z FC |
| C / CUR | wyjście czujnika prądu, jeśli obsługiwane |
| NC | niepodłączone |

### Połączenia ESC

| ESC | Połączenie |
|---|---|
| BAT+ | XT60 + baterii |
| BAT- | XT60 - baterii |
| M1 | silnik 1, trzy przewody |
| M2 | silnik 2, trzy przewody |
| M3 | silnik 3, trzy przewody |
| M4 | silnik 4, trzy przewody |
| GND | GND BlackPill |
| S1 | pin M1 BlackPill |
| S2 | pin M2 BlackPill |
| S3 | pin M3 BlackPill |
| S4 | pin M4 BlackPill |
| VT / VBAT | dzielnik napięcia -> ADC BlackPill, opcjonalnie |
| C / CUR | ADC BlackPill, opcjonalnie po ustaleniu skali |

---

## 11. Silniki

### RS2205 2300KV Brushless Motor 3–4S

| Parametr | Wartość |
|---|---|
| Typ | Silnik bezszczotkowy outrunner |
| Liczba sztuk dla quada | 4 |
| KV | 2300 KV |
| Zasilanie | 3–4S LiPo |
| Stator | 22 × 5 mm |
| Konfiguracja | 12N14P |
| Średnica silnika | ok. 27.9 mm |
| Wysokość silnika | ok. 31.7 mm |
| Masa | ok. 30 g z przewodami |
| Przewody | 20 AWG według danych EMAX RS2205 |
| Wał / gwint adaptera śmigła | M5 |
| Średnica wału | 3 mm |
| Rekomendowane śmigła | 5045 / 5" |
| Dokumentacja / strona producenta | [EMAX — RS2205 RaceSpec Motor](https://emaxmodel.com/products/emax-rs2205-racespec-motor-cooling-series) |

### Połączenia silników

| Silnik | Połączenie |
|---|---|
| Silnik 1 | trzy przewody -> pady M1 ESC |
| Silnik 2 | trzy przewody -> pady M2 ESC |
| Silnik 3 | trzy przewody -> pady M3 ESC |
| Silnik 4 | trzy przewody -> pady M4 ESC |

---

## 12. BEC / regulator 5 V

### BEC 5 V 2–3 A, wejście 2–6S

| Parametr | Wartość docelowa |
|---|---|
| Typ | przetwornica DC-DC / BEC |
| Wejście | 2–6S LiPo |
| Zakres wejściowy wymagany dla 4S | minimum do 16.8 V |
| Wyjście | 5 V |
| Prąd wyjściowy | 2–3 A |
| Dokumentacja | zależna od wybranego modelu |

### Połączenia BEC

| BEC | Połączenie |
|---|---|
| IN+ | XT60 + / BAT+ |
| IN- | XT60 - / BAT- |
| OUT 5V | BlackPill 5V |
| OUT 5V | odbiornik ELRS 5V |
| OUT GND | wspólna masa |

---

## 13. Kondensator wejściowy ESC

### Kondensator low-ESR

| Parametr | Wartość |
|---|---|
| Typ | elektrolityczny low-ESR |
| Pojemność | do potwierdzenia z nadruku; typowo 470–1000 µF |
| Napięcie | do potwierdzenia z nadruku; dla 4S minimum 25 V, typowo 35 V |
| Montaż | równolegle do BAT+ i BAT- ESC |
| Dokumentacja | zależna od konkretnego kondensatora |

### Połączenia kondensatora

| Kondensator | Połączenie |
|---|---|
| + | ESC BAT+ |
| - | ESC BAT- |

---

## 14. Dzielnik napięcia VBAT do ADC

### Dzielnik pomiarowy baterii 4S

| Parametr | Wartość przykładowa |
|---|---|
| Wejście | VBAT 4S, maks. 16.8 V |
| Wyjście | ADC STM32, maks. 3.3 V |
| Rezystor górny | 100 kΩ |
| Rezystor dolny | 20 kΩ |
| Napięcie ADC przy 16.8 V | ok. 2.8 V |
| Kondensator filtrujący | 100 nF do GND, opcjonalnie |

### Połączenia dzielnika

| Punkt | Połączenie |
|---|---|
| VBAT | rezystor 100 kΩ |
| Punkt środkowy | ADC BlackPill |
| Punkt środkowy | kondensator 100 nF do GND, opcjonalnie |
| GND | rezystor 20 kΩ |

---

## 15. Zbiorczy schemat połączeń

```text
Bateria 4S XT60
  |
  +--> ESC BAT+ / BAT-
  |       |
  |       +--> Silnik 1 -> M1
  |       +--> Silnik 2 -> M2
  |       +--> Silnik 3 -> M3
  |       +--> Silnik 4 -> M4
  |
  +--> BEC 5V
          |
          +--> BlackPill 5V
          +--> ELRS 5V

ESC GND ----------- BlackPill GND
BEC GND ----------- BlackPill GND
ELRS GND ---------- BlackPill GND
Czujniki GND ------ BlackPill GND

BlackPill M1 ------ ESC S1
BlackPill M2 ------ ESC S2
BlackPill M3 ------ ESC S3
BlackPill M4 ------ ESC S4

BlackPill 3V3 ----- MPU6050 VCC
BlackPill 3V3 ----- BMP280 VCC
BlackPill 3V3 ----- QMC5883L VCC

BlackPill SDA ----- MPU6050 SDA / BMP280 SDA / QMC5883L SDA
BlackPill SCL ----- MPU6050 SCL / BMP280 SCL / QMC5883L SCL

ELRS TX ----------- BlackPill RX
ELRS RX ----------- BlackPill TX

ESC VT/VBAT ------- dzielnik napięcia ------- BlackPill ADC
ESC CUR ----------- BlackPill ADC, opcjonalnie

Kondensator + ----- ESC BAT+
Kondensator - ----- ESC BAT-
