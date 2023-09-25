// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![forbid(unsafe_code)]
#![no_std]

pub mod test;
pub mod tutorials;

#[macro_use]
pub mod net;

pub mod adc_microphone;
pub mod air_quality;
pub mod ambient_light;
pub mod analog_comparator;
pub mod analog_sensor;
pub mod apds9960;
pub mod app_flash_driver;
pub mod at24c_eeprom;
pub mod ble_advertising_driver;
pub mod bme280;
pub mod bmp280;
pub mod bus;
pub mod buzzer_driver;
pub mod buzzer_pwm;
pub mod can;
pub mod ccs811;
pub mod crc;
pub mod dac;
pub mod debug_process_restart;
pub mod fm25cl;
pub mod ft6x06;
pub mod fxos8700cq;
pub mod gpio_async;
pub mod hd44780;
pub mod hmac;
pub mod hmac_sha256;
pub mod hs3003;
pub mod hts221;
pub mod humidity;
pub mod ieee802154;
pub mod isl29035;
pub mod kv_driver;
pub mod kv_store_permissions;
pub mod l3gd20;
pub mod led_matrix;
pub mod log;
pub mod lpm013m126;
pub mod lps25hb;
pub mod lsm303agr;
pub mod lsm303dlhc;
pub mod lsm303xx;
pub mod lsm6dsoxtr;
pub mod ltc294x;
pub mod max17205;
pub mod mcp230xx;
pub mod mlx90614;
pub mod mx25r6435f;
pub mod ninedof;
pub mod nonvolatile_storage_driver;
pub mod nonvolatile_to_pages;
pub mod nrf51822_serialization;
pub mod panic_button;
pub mod pca9544a;
pub mod pressure;
pub mod proximity;
pub mod public_key_crypto;
pub mod pwm;
pub mod read_only_state;
pub mod rf233;
pub mod rf233_const;
pub mod screen;
pub mod sdcard;
pub mod segger_rtt;
pub mod seven_segment;
pub mod sha;
pub mod sha256;
pub mod sht3x;
pub mod si7021;
pub mod sip_hash;
pub mod sound_pressure;
pub mod st77xx;
pub mod symmetric_encryption;
pub mod temperature;
pub mod temperature_rp2040;
pub mod temperature_stm;
pub mod text_screen;
pub mod tickv;
pub mod tickv_kv_store;
pub mod touch;
pub mod tsl2561;
pub mod usb;
pub mod usb_hid_driver;
pub mod virtual_kv;
