#![allow(dead_code)]

// Addresses of the registers in the AS8510
pub(crate) const AS8510_DREG_I1: u8 = 0x00; // Current data registers
pub(crate) const AS8510_DREG_I2: u8 = 0x01;
pub(crate) const AS8510_DREG_V1: u8 = 0x02; // Voltage data registers
pub(crate) const AS8510_DREG_V2: u8 = 0x03;
pub(crate) const AS8510_STATUS_REG: u8 = 0x04; // Read-only status register
pub(crate) const AS8510_DEC_REG_R1_I: u8 = 0x05; // Control register 1
pub(crate) const AS8510_DEC_REG_R2_I: u8 = 0x06; // Control register 2
pub(crate) const AS8510_FIR_CTL_REG_I: u8 = 0x07;
pub(crate) const AS8510_CLK_REG: u8 = 0x08; // Clock control register
pub(crate) const AS8510_RESET_REG: u8 = 0x09; // Soft-reset control
pub(crate) const AS8510_MOD_CTL_REG: u8 = 0x0A; // Mode control register
pub(crate) const AS8510_MOD_TA_REG1: u8 = 0x0B; // Timing control for SBM1/SBM2
pub(crate) const AS8510_MOD_TA_REG2: u8 = 0x0C;
pub(crate) const AS8510_MOD_ITH_REG1: u8 = 0x0D; // Threshold control for SBM1/SBM2
pub(crate) const AS8510_MOD_ITH_REG2: u8 = 0x0E;
pub(crate) const AS8510_MOD_TMC_REG1: u8 = 0x0F; // Configures number of ADC samples to drop
pub(crate) const AS8510_MOD_TMC_REG2: u8 = 0x10;
pub(crate) const AS8510_NOM_ITH_REG1: u8 = 0x11; // Threshold control for NOM2
pub(crate) const AS8510_NOM_ITH_REG2: u8 = 0x12;
pub(crate) const AS8510_PGA_CTL_REG: u8 = 0x13; // Gain control
pub(crate) const AS8510_PD_CTL_REG_1: u8 = 0x14; // Power control registers
pub(crate) const AS8510_PD_CTL_REG_2: u8 = 0x15;
pub(crate) const AS8510_PD_CTL_REG_3: u8 = 0x16;
pub(crate) const AS8510_ACH_CTL_REG: u8 = 0x17; // Analog channel selection
pub(crate) const AS8510_ISC_CTL_REG: u8 = 0x18; // Current source setting register (internal source)
pub(crate) const AS8510_OTP_EN_REG: u8 = 0x19; // Reserved
pub(crate) const AS8510_STATUS_REG_2: u8 = 0x44; // Data saturation flags
pub(crate) const AS8510_DEC_R1_V: u8 = 0x45; // Voltage control registers
pub(crate) const AS8510_DEC_R2_V: u8 = 0x46;
pub(crate) const AS8510_FIR_CTL_REG_V: u8 = 0x47;
pub(crate) const CALIBRATION: u16 = 0x3670;
pub(crate) const ADDR_CURRENT: u8 = 0x0;
pub(crate) const ADDR_VOLTAGE: u8 = 0x2;
pub(crate) const ADDR_STATUS: u8 = 0x5;
