#![no_std]
use consts::*;
/// Reads bi-directional current from the AS8510 sensor.
///
/// # Example
///
/// ```no_run
/// use esp_hal::spi::{Config, SpiDeviceWithConfig, Rate};
/// use core::time::Duration;
///
/// let spi_bus: Mutex<_, esp_hal::spi::master::SpiDmaBus<'_, esp_hal::Async>> = Mutex::new(spi);
///
/// let spi_device_2_config = Config::default()
///     .with_frequency(Rate::from_khz(1000))
///     .with_mode(esp_hal::spi::Mode::_1);
///
/// let spi_device_2 = SpiDeviceWithConfig::new(&spi_bus, ss_2, spi_device_2_config);
///
/// if let Ok(mut device) = as8510::As8510::new(spi_device_2, as8510::Gain::Gain100, as8510::Gain::Gain25).await {
///     loop {
///         match device.get_current().await {
///             Ok(amps) => info!("Amps: {}A", amps),
///             Err(e) => warn!("Failed to read current: {:?}", e),
///         }
///     }
/// }
/// ```
use core::fmt::Debug;
use embedded_hal_async::spi::{self, Operation};
use errors::As8510Error;
mod consts;
mod errors;

/// Max readings
/// Gain1 +2076/-1523A
/// Gain25 +/- 400A
/// Gain40 +/- 235A
/// Gain100 +/- 77A
#[derive(Clone, Copy)]
pub enum Gain {
    Gain1 = 1,
    Gain25 = 25,
    Gain40 = 40,
    Gain100 = 100,
}

impl From<Gain> for i32 {
    fn from(gain: Gain) -> Self {
        gain as i32
    }
}

pub struct As8510<SPI> {
    peri: SPI,
    data: [u8; 5],
    current_gain: Gain,
    voltage_gain: Gain,
}

impl<SPI, E> As8510<SPI>
where
    SPI: spi::SpiDevice<u8, Error = E>,
    As8510Error: From<E>,
{
    pub async fn new(
        peri: SPI,
        current_gain: Gain,
        voltage_gain: Gain,
    ) -> Result<Self, As8510Error> {
        let mut device = Self {
            peri,
            data: [0; 5],
            current_gain,
            voltage_gain,
        };

        assert!(
            matches!(voltage_gain, Gain::Gain40) | matches!(voltage_gain, Gain::Gain25),
            "Voltage gain must be Gain25 or Gain40"
        );

        device.write(AS8510_DEC_REG_R1_I, 0b0100_0101).await?;
        device.write(AS8510_DEC_REG_R2_I, 0b1100_0101).await?;
        device.write(AS8510_FIR_CTL_REG_I, 0b0000_0100).await?;
        device.write(AS8510_CLK_REG, 0b0010_0000).await?;
        device.write(AS8510_MOD_CTL_REG, 0).await?;
        device.write(AS8510_MOD_TA_REG1, 0b1000_0000).await?;
        device.write(AS8510_MOD_TA_REG2, 0).await?;
        device.write(AS8510_MOD_ITH_REG1, 0b0101_0000).await?;
        device.write(AS8510_MOD_ITH_REG2, 0b1100_1111).await?;
        device.write(AS8510_MOD_TMC_REG1, 0b1111_0011).await?;
        device.write(AS8510_MOD_TMC_REG2, 0b1111_1000).await?;
        device.write(AS8510_NOM_ITH_REG1, 0).await?;
        device.write(AS8510_NOM_ITH_REG2, 0).await?;
        device.write(AS8510_PGA_CTL_REG, device.reg_0x13()).await?;
        device.write(AS8510_PD_CTL_REG_1, 0b1100_1111).await?;
        device.write(AS8510_PD_CTL_REG_2, device.reg_0x15()).await?;
        device.write(AS8510_PD_CTL_REG_3, 0b1111_1000).await?;
        device.write(AS8510_ACH_CTL_REG, 0).await?;
        device.write(AS8510_ISC_CTL_REG, 0).await?;
        device.write(AS8510_OTP_EN_REG, 0).await?;

        // Start read
        device.write(AS8510_MOD_CTL_REG, 0x01).await?;
        Ok(device)
    }

    #[inline]
    fn reg_0x15(&self) -> u8 {
        0xf0 | if matches!(self.current_gain, Gain::Gain1) {
            0xf
        } else {
            0b0010
        } | if matches!(self.voltage_gain, Gain::Gain1) {
            0
        } else {
            0b0001
        }
    }

    #[inline]
    fn reg_0x13(&self) -> u8 {
        (match self.current_gain {
            Gain::Gain1 => 0,
            Gain::Gain25 => 0b0100,
            Gain::Gain40 => 0b1000,
            Gain::Gain100 => 0b1100,
        } | match self.voltage_gain {
            Gain::Gain1 => 0,
            Gain::Gain25 => 0b0001,
            Gain::Gain40 => 0b001,
            Gain::Gain100 => 0b0011,
        }) << 4
    }

    async fn write(&mut self, addr: u8, data: u8) -> Result<(), As8510Error> {
        if addr > 0x47 {
            return Err(As8510Error::IllegalAddress(addr));
        }
        let txdata = [addr, data];
        self.peri
            .transaction(&mut [Operation::Write(&txdata)])
            .await?;
        Ok(())
    }

    async fn read(&mut self, addr: u8, len: u8) -> Result<&[u8], As8510Error> {
        if addr + len > 0x48 {
            return Err(As8510Error::IllegalAddress(addr + len));
        }
        self.data.fill(0);
        let txaddr = addr | 0x80;
        let len = len as usize;
        self.peri
            .transaction(&mut [
                Operation::Write(&[txaddr]),
                Operation::Read(&mut self.data[..len]),
            ])
            .await?;
        Ok(&self.data[..len])
    }

    pub async fn get_current(&mut self) -> Result<f32, As8510Error> {
        if !self.get_status().await?.current_channel_updated() {
            return Err(As8510Error::NotReady);
        }
        self.read(ADDR_CURRENT, 2).await?;

        let adc =
            i16::from_be_bytes([self.data[0], self.data[1]]).wrapping_add(0x8000u16 as i16) as i32;
        let val = ((adc * CALIBRATION as i32) / i32::from(self.current_gain) / 2000) as f32 * 0.1;
        Ok(val)
    }

    /// Unverified
    pub async fn get_voltage(&mut self) -> Result<f32, As8510Error> {
        if !self.get_status().await?.voltage_channel_updated() {
            return Err(As8510Error::NotReady);
        }
        self.read(ADDR_VOLTAGE, 2).await?;
        let adc =
            i16::from_be_bytes([self.data[0], self.data[1]]).wrapping_add(0x8000u16 as i16) as i32;
        let val = ((adc * CALIBRATION as i32) / i32::from(self.current_gain) / 2000) as f32 * 0.1;
        Ok(val)
    }

    async fn get_status(&mut self) -> Result<StatusReg, As8510Error> {
        self.read(ADDR_STATUS, 1).await?;
        Ok(StatusReg::new(self.data[0]))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct StatusReg(u8);

impl StatusReg {
    pub const CURRENT_CHANNEL_UPDATED: u8 = 1 << 2;
    pub const VOLTAGE_CHANNEL_UPDATED: u8 = 1 << 1;

    pub fn new(value: u8) -> Self {
        StatusReg(value)
    }

    pub fn current_channel_updated(&self) -> bool {
        self.0 & Self::CURRENT_CHANNEL_UPDATED != 0
    }

    pub fn voltage_channel_updated(&self) -> bool {
        self.0 & Self::VOLTAGE_CHANNEL_UPDATED != 0
    }
}
