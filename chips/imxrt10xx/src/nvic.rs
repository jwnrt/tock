// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Named constants for NVIC ids

// Enabling only the needed constants

pub const DMA0_16: u32 = 0;
pub const DMA1_17: u32 = 1;
pub const DMA2_18: u32 = 2;
pub const DMA3_19: u32 = 3;
pub const DMA4_20: u32 = 4;
pub const DMA5_21: u32 = 5;
pub const DMA6_22: u32 = 6;
pub const DMA7_23: u32 = 7;
pub const DMA8_24: u32 = 8;
pub const DMA9_25: u32 = 9;
pub const DMA10_26: u32 = 10;
pub const DMA11_27: u32 = 11;
pub const DMA12_28: u32 = 12;
pub const DMA13_29: u32 = 13;
pub const DMA14_30: u32 = 14;
pub const DMA15_31: u32 = 15;
pub const DMA_ERROR: u32 = 16;
// pub const CM7: u32 = 17;
// pub const CM7: u32 = 18;
// pub const CM7: u32 = 19;
pub const LPUART1: u32 = 20;
pub const LPUART2: u32 = 21;
// pub const LPUART3: u32 = 22;
// pub const LPUART4: u32 = 23;
// pub const LPUART5: u32 = 24;
// pub const LPUART6: u32 = 25;
// pub const LPUART7: u32 = 26;
// pub const LPUART8: u32 = 27;
pub const LPI2C1: u32 = 28;
// pub const LPI2C2: u32 = 29;
// pub const LPI2C3: u32 = 30;
// pub const LPI2C4: u32 = 31;
// pub const LPSPI1: u32 = 32;
// pub const LPSPI2: u32 = 33;
// pub const LPSPI3: u32 = 34;
// pub const LPSPI4: u32 = 35;
// pub const FLEXCAN1: u32 = 36;
// pub const FLEXCAN2: u32 = 37;
// pub const CM7: u32 = 38;
// pub const KPP: u32 = 39;
// pub const TSC_DIG: u32 = 40;
// pub const GPR_IRQ: u32 = 41;
// pub const LCDIF: u32 = 42;
// pub const CSI: u32 = 43;
// pub const PXP: u32 = 44;
// pub const WDOG2: u32 = 45;
// pub const SNVS_HP_WRAPPER: u32 = 46;
// pub const SNVS_HP_WRAPPER: u32 = 47;
pub const SNVS_LP_WRAPPER: u32 = 48;
// pub const CSU: u32 = 49;
// pub const DCP: u32 = 50;
// pub const DCP: u32 = 51;
// pub const DCP: u32 = 52;
// pub const TRNG: u32 = 53;
// pub const BEE: u32 = 55;
// pub const SAI1: u32 = 56;
// pub const SAI2: u32 = 57;
// pub const SAI3: u32 = 58;
// pub const SAI3: u32 = 59;
// pub const SPDIF: u32 = 60;
// pub const PMU: u32 = 61;
// pub const Temperature_Monitor: u32 = 63;
// pub const Temperature_Monitor: u32 = 64;
// pub const USB_PHY: u32 = 65;
// pub const USB_PHY: u32 = 66;
// pub const ADC1: u32 = 67;
// pub const ADC2: u32 = 68;
// pub const DCDC: u32 = 69;
// pub const GPIO1: u32 = 72;
// pub const GPIO1: u32 = 73;
// pub const GPIO1: u32 = 74;
// pub const GPIO1: u32 = 75;
// pub const GPIO1: u32 = 76;
// pub const GPIO1: u32 = 77;
// pub const GPIO1: u32 = 78;
// pub const GPIO1: u32 = 79;
pub const GPIO1_1: u32 = 80;
pub const GPIO1_2: u32 = 81;
pub const GPIO2_1: u32 = 82;
pub const GPIO2_2: u32 = 83;
pub const GPIO3_1: u32 = 84;
pub const GPIO3_2: u32 = 85;
pub const GPIO4_1: u32 = 86;
pub const GPIO4_2: u32 = 87;
pub const GPIO5_1: u32 = 88;
pub const GPIO5_2: u32 = 89;
// pub const FLEXIO1: u32 = 90;
// pub const FLEXIO2: u32 = 91;
// pub const WDOG1: u32 = 92;
// pub const RTWDOG: u32 = 93;
// pub const EWM: u32 = 94;
// pub const CCM: u32 = 95;
// pub const CCM: u32 = 96;
// pub const GPC: u32 = 97;
// pub const SRC: u32 = 98;
pub const GPT1: u32 = 100;
pub const GPT2: u32 = 101;
// pub const FLEXPWM1: u32 = 102;
// pub const FLEXPWM1: u32 = 103;
// pub const FLEXPWM1: u32 = 104;
// pub const FLEXPWM1: u32 = 105;
// pub const FLEXPWM1: u32 = 106;
// pub const FLEXSPI: u32 = 108;
// pub const SEMC: u32 = 109;
// pub const USDHC1: u32 = 110;
// pub const USDHC2: u32 = 111;
// pub const USB: u32 = 112;
// pub const USB: u32 = 113;
// pub const ENET: u32 = 114;
// pub const ENET: u32 = 115;
// pub const XBAR1: u32 = 116;
// pub const XBAR1: u32 = 117;
// pub const ADC_ETC: u32 = 118;
// pub const ADC_ETC: u32 = 119;
// pub const ADC_ETC: u32 = 120;
// pub const ADC_ETC: u32 = 121;
// pub const PIT: u32 = 122;
// pub const ACMP: u32 = 123;
// pub const ACMP: u32 = 124;
// pub const ACMP: u32 = 125;
// pub const ACMP: u32 = 126;
// pub const ENC1: u32 = 129;
// pub const ENC2: u32 = 130;
// pub const ENC3: u32 = 131;
// pub const ENC4: u32 = 132;
// pub const QTIMER1: u32 = 133;
// pub const QTIMER2: u32 = 134;
// pub const QTIMER3: u32 = 135;
// pub const QTIMER4: u32 = 136;
// pub const FLEXPWM2: u32 = 137;
// pub const FLEXPWM2: u32 = 138;
// pub const FLEXPWM2: u32 = 139;
// pub const FLEXPWM2: u32 = 140;
// pub const FLEXPWM2: u32 = 141;
// pub const FLEXPWM3: u32 = 142;
// pub const FLEXPWM3: u32 = 143;
// pub const FLEXPWM3: u32 = 144;
// pub const FLEXPWM3: u32 = 145;
// pub const FLEXPWM3: u32 = 146;
// pub const FLEXPWM4: u32 = 147;
// pub const FLEXPWM4: u32 = 148;
// pub const FLEXPWM4: u32 = 149;
// pub const FLEXPWM4: u32 = 150;
// pub const FLEXPWM4: u32 = 151;
