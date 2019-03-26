//! # uFire Isolated EC Probe Interface
//!
//! * measure EC
//! * measure salinity
//! * temperature in Celsius

use std::f32;
use std::thread;
use std::time::Duration;
use std::mem;

extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};

extern crate i2cdev;
use self::i2cdev::core::*;
use self::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const EC_MEASURE_EC: u8 = 80;
const EC_MEASURE_TEMP: u8 = 40;
const EC_CALIBRATE_PROBE: u8 = 20;
const EC_CALIBRATE_LOW: u8 = 10;
const EC_CALIBRATE_HIGH: u8 = 8;
const EC_I2C: u8 = 4;
const EC_READ: u8 = 2;
const EC_WRITE: u8 = 1;

const EC_VERSION_REGISTER: u8 = 0;
const EC_MS_REGISTER: u8 = 1;
const EC_TEMP_REGISTER: u8 = 5;
const EC_SOLUTION_REGISTER: u8 = 9;
const EC_TEMPCOEF_REGISTER: u8 = 13;
const EC_CALIBRATE_REFHIGH_REGISTER: u8 = 17;
const EC_CALIBRATE_REFLOW_REGISTER: u8 = 21;
const EC_CALIBRATE_READHIGH_REGISTER: u8 = 25;
const EC_CALIBRATE_READLOW_REGISTER: u8 = 29;
const EC_CALIBRATE_OFFSET_REGISTER: u8 = 33;
const EC_PSU_REGISTER: u8 = 37;
const EC_RAW_REGISTER: u8 = 41;
const EC_TEMP_COMPENSATION_REGISTER: u8 = 45;
const EC_BUFFER_REGISTER: u8 = 49;
const EC_FW_VERSION_REGISTER: u8 = 53;
const EC_CONFIG_REGISTER: u8 = 54;
const EC_TASK_REGISTER: u8 = 55;    

const EC_TEMP_COMPENSATION_CONFIG_BIT: u8 = 1;

const EC_EC_MEASUREMENT_TIME: u64 = 750;
const EC_TEMP_MEASURE_TIME: u64 = 750;

pub struct EcProbe {
    dev: Box<LinuxI2CDevice>,
}

impl EcProbe {
    /// Create a new EcProbe object
    ///
    /// Pass the i2c port to use, it must be a software overlay device, and I2C address.
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ```
    pub fn new(filename: &'static str, address: u16) -> Result<Self, Box<LinuxI2CError>> {
        let dev = LinuxI2CDevice::new(filename, address)?;
        Ok(EcProbe { dev: Box::new(dev) })
    }

    /// Start a temperature measurement
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.measure_temp();
    /// ```
    pub fn measure_temp(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_MEASURE_TEMP)?;
        thread::sleep(Duration::from_millis(EC_TEMP_MEASURE_TIME));

        Ok(self._read_register(EC_TEMP_REGISTER)?)
    }

    /// Sets the temperature used by the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_temp(20.2);
    /// ```
    pub fn set_temp(&mut self, temp_c: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_TEMP_REGISTER, temp_c)?;

        Ok(())
    }

    /// Calibrates the probe using a single point using a mS value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.calibrate_single(1.413.0);
    /// ```
    pub fn calibrate_single(&mut self, solution_ec: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, solution_ec)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_CALIBRATE_PROBE)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));
        Ok(())
    }
    
    /// Calibrates the dual-point values for the low reading, in mS, and saves them in the
    /// devices's EEPROM.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.calibrate_probe_low(1.413.0);
    /// ```
    pub fn calibrate_probe_low(&mut self, solution_ec: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, solution_ec)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_CALIBRATE_LOW)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));
        Ok(())
    }

    /// Calibrates the dual-point values for the high reading, in mS, and saves them in the
    /// devices's EEPROM.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.calibrate_probe_high(12.88.0);
    /// ```
    pub fn calibrate_probe_high(&mut self, solution_ec: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, solution_ec)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_CALIBRATE_HIGH)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));
        Ok(())
    }


    /// Starts an EC measurement, taking a new temp measurement if true passed
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.measure_ec(22.1);
    /// ```
    pub fn measure_ec(&mut self, temp_c: f32) -> Result<(f32), Box<LinuxI2CError>> {
	self.use_temperature_compensation(true)?;
        self._write_register(EC_TEMP_REGISTER, temp_c)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_MEASURE_EC)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));

        Ok(self._read_register(EC_MS_REGISTER)?)
    }

    /// Starts an salinity measurement and returns the salinity in PSU.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.measure_salinity(22.1);
    /// ```
    pub fn measure_salinity(&mut self, temp_c: f32) -> Result<(f32), Box<LinuxI2CError>> {
        self._write_register(EC_TEMP_REGISTER, temp_c)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_MEASURE_EC)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));
        Ok(self._read_register(EC_PSU_REGISTER)?)
    }

    /// Configures the device to use temperature compensation or not.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.use_temperature_compensation(true);
    /// assert_eq!(1, ec.using_temperature_compensation().unwrap());
    /// ```
    pub fn use_temperature_compensation(&mut self, b: bool) -> Result<(), Box<LinuxI2CError>> {
        self._change_register(EC_CONFIG_REGISTER)?;
        let mut config: u8 = self.dev.smbus_read_byte()?;
        thread::sleep(Duration::from_millis(10));
        if b {
            config |= 1 << EC_TEMP_COMPENSATION_CONFIG_BIT;
        } else {
            config &= !(1 << EC_TEMP_COMPENSATION_CONFIG_BIT);
        }
        self.dev.smbus_write_byte_data(EC_CONFIG_REGISTER, config)?;
        thread::sleep(Duration::from_millis(10));
        Ok(())
   }

    /// Starts a raw measurement.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.measure_raw(true);
    /// ```
    pub fn measure_raw(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_MEASURE_EC)?;
        thread::sleep(Duration::from_millis(EC_EC_MEASUREMENT_TIME));

        Ok(self._read_register(EC_RAW_REGISTER)?)
    }

    /// Sets the temperature constant to use for compensation and saves it in the devices's EEPROM.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_temp_constant(20);
    /// ```
    pub fn set_temp_constant(&mut self, temp_constant: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_TEMP_COMPENSATION_REGISTER, temp_constant)?;

        Ok(())
    }

    /// Returns the temperature constant from the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.get_temp_constant();
    /// ```
    pub fn get_temp_constant(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_TEMP_COMPENSATION_REGISTER)?)
    }

    /// Sets the temperature coefficient to use for compensation and saves it in the devices's EEPROM.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_temp_coefficient(0.019);
    /// ```
    pub fn set_temp_coefficient(&mut self, temp_coef: f32) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_TEMPCOEF_REGISTER, temp_coef)?;

        Ok(())
    }

    /// Returns the temperature coefficient from the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.get_temp_coefficient();
    /// ```
    pub fn get_temp_coefficient(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_TEMPCOEF_REGISTER)?)
    }

    /// Returns the dual-point calibration low reference value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(50.0, 0.0, 0.0, 0.0);
    /// assert_eq!(50.0, ec.get_calibrate_low_reference().unwrap());
    /// ```
    pub fn read_eeprom(&mut self, address: f32) -> Result<(f32), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, address)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_READ)?;
        Ok(self._read_register(EC_BUFFER_REGISTER)?)
    }

    /// Returns the dual-point calibration low reference value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(50.0, 0.0, 0.0, 0.0);
    /// assert_eq!(50.0, ec.get_calibrate_low_reference().unwrap());
    /// ```
    pub fn write_eeprom(&mut self, address: f32, value: f32) -> Result<(f32), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, address)?;
        self._write_register(EC_BUFFER_REGISTER, value)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_WRITE)?;
        Ok(self._read_register(EC_BUFFER_REGISTER)?)
    }

    /// Returns the firmware version of the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// assert_eq!(0x1c, ec.get_version().unwrap());
    /// ```
    pub fn get_version(&mut self) -> Result<(u8), Box<LinuxI2CError>> {
        self._change_register(EC_VERSION_REGISTER)?;
        Ok(self.dev.smbus_read_byte()?)
    }

    /// Returns the firmware version of the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// assert_eq!(0x1, ec.get_version().unwrap());
    /// ```
    pub fn get_firmware(&mut self) -> Result<(u8), Box<LinuxI2CError>> {
        self._change_register(EC_FW_VERSION_REGISTER)?;
        Ok(self.dev.smbus_read_byte()?)
    }

    /// Resets all the stored calibration information.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.reset();
    /// assert_eq!(true, ec.get_calibrate_offset().unwrap().is_nan());
    /// assert_eq!(true, ec.get_calibrate_low_reading().unwrap().is_nan());
    /// assert_eq!(true, ec.get_calibrate_high_reading().unwrap().is_nan());
    /// assert_eq!(true, ec.get_calibrate_low_reference().unwrap().is_nan());
    /// assert_eq!(true, ec.get_calibrate_high_reference().unwrap().is_nan());
    /// ```
    pub fn reset(&mut self) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_CALIBRATE_REFLOW_REGISTER, f32::NAN)?;
        self._write_register(EC_CALIBRATE_REFHIGH_REGISTER, f32::NAN)?;
        self._write_register(EC_CALIBRATE_READLOW_REGISTER, f32::NAN)?;
        self._write_register(EC_CALIBRATE_READHIGH_REGISTER, f32::NAN)?;
        self._write_register(EC_CALIBRATE_OFFSET_REGISTER, f32::NAN)?;
        self._write_register(EC_TEMP_COMPENSATION_REGISTER, 25.0)?;
	    self.set_temp_coefficient(0.019)?;
        Ok(())
    }

    /// Sets the I2C address of the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// // ec.set_i2c_address(0x4f);
    /// ```
    pub fn set_i2c_address(&mut self, i2c_address: u16) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_SOLUTION_REGISTER, i2c_address as f32)?;
        self.dev.smbus_write_byte_data(EC_TASK_REGISTER, EC_I2C)?;

        Ok(())
    }

    /// Sets all the values, in mS, for dual point calibration and saves them in the devices's
    /// EEPROM.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(50.0, 58.0, 48.0, 56.0);
    /// ```
    pub fn set_dual_point_calibration(
        &mut self,
        ref_low: f32,
        ref_high: f32,
        read_low: f32,
        read_high: f32,
    ) -> Result<(), Box<LinuxI2CError>> {
        self._write_register(EC_CALIBRATE_REFLOW_REGISTER, ref_low)?;
        self._write_register(EC_CALIBRATE_REFHIGH_REGISTER, ref_high)?;
        self._write_register(EC_CALIBRATE_READLOW_REGISTER, read_low)?;
        self._write_register(EC_CALIBRATE_READHIGH_REGISTER, read_high)?;
        Ok(())
    }

    /// Returns the single-point offset from the device.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.get_calibrate_offset();
    /// ```
    pub fn get_calibrate_offset(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_CALIBRATE_OFFSET_REGISTER)?)
    }

    /// Returns the dual-point calibration high reference value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(0.0, 500.0, 0.0, 0.0);
    /// ```
    pub fn get_calibrate_high_reference(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_CALIBRATE_REFHIGH_REGISTER)?)
    }
    /// Returns the dual-point calibration high reading value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(0.0, 0.0, 0.0, 553.0);
    /// assert_eq!(553.0, ec.get_calibrate_high_reading().unwrap());
    /// ```
    pub fn get_calibrate_high_reading(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_CALIBRATE_READHIGH_REGISTER)?)
    }
    /// Returns the dual-point calibration low reference value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(50.0, 0.0, 0.0, 0.0);
    /// assert_eq!(50.0, ec.get_calibrate_low_reference().unwrap());
    /// ```
    pub fn get_calibrate_low_reference(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_CALIBRATE_REFLOW_REGISTER)?)
    }
    /// Returns the dual-point calibration low reading value.
    ///
    /// # Example
    /// ```
    /// let mut ec = ufire_ec::EcProbe::new("/dev/i2c-3", 0x3c).unwrap();
    /// ec.set_dual_point_calibration(0.0, 0.0, 34.0, 0.0);
    /// assert_eq!(34.0, ec.get_calibrate_low_reading().unwrap());
    /// ```
    pub fn get_calibrate_low_reading(&mut self) -> Result<(f32), Box<LinuxI2CError>> {
        Ok(self._read_register(EC_CALIBRATE_READLOW_REGISTER)?)
    }
    
    pub fn _write_register(&mut self, register: u8, f_val: f32) -> Result<(), Box<LinuxI2CError>> {
        unsafe {
            let buf: [u8; 4] = mem::transmute(f_val);
            self._change_register(register)?;
            self.dev.smbus_write_byte_data(register + 0, buf[0])?;
            thread::sleep(Duration::from_millis(10));
            self.dev.smbus_write_byte_data(register + 1, buf[1])?;
            thread::sleep(Duration::from_millis(10));
            self.dev.smbus_write_byte_data(register + 2, buf[2])?;
            thread::sleep(Duration::from_millis(10));
            self.dev.smbus_write_byte_data(register + 3, buf[3])?;
            thread::sleep(Duration::from_millis(10));
            Ok(())
        }
    }

    pub fn _read_register(&mut self, register: u8) -> Result<(f32), Box<LinuxI2CError>> {
        let mut buf: [u8; 4] = [0; 4];
        self._change_register(register)?;
        buf[0] = self.dev.smbus_read_byte()?;
        thread::sleep(Duration::from_millis(10));
        buf[1] = self.dev.smbus_read_byte()?;
        thread::sleep(Duration::from_millis(10));
        buf[2] = self.dev.smbus_read_byte()?;
        thread::sleep(Duration::from_millis(10));
        buf[3] = self.dev.smbus_read_byte()?;
        thread::sleep(Duration::from_millis(10));
        Ok(LittleEndian::read_f32(&buf))
    }

    pub fn _change_register(&mut self, register: u8) -> Result<(), Box<LinuxI2CError>> {
        self.dev.smbus_write_byte(register)?;
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}
