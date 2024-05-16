use esp32_hal::{
    delay::Delay,
    i2c::{Error, I2C},
    peripherals::I2C0,
    prelude::*,
};
use embedded_hal::blocking::i2c::{Read, Write};

const CONFIG: u8 = 0x01;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL: u8 = 0x3B;
const TEMP: u8 = 0x41;
const GYRO: u8 = 0x43;
const SLAVE_ADDR: u8 = 0x68;

pub fn imu_init(mpu: &mut I2C<I2C0>, delay: &mut Delay) -> Result<(), Error> {
    mpu.write(SLAVE_ADDR, &[PWR_MGMT_1, 0b1_0_0_0_0_000])?; // reset
    delay.delay_ms(1000u32);
    mpu.write(SLAVE_ADDR, &[PWR_MGMT_1, 0b0_0_0_0_0_001])?; // enable gyroscope
    mpu.write(SLAVE_ADDR, &[CONFIG, 0b0_0_0_00_0_01])?;

    Ok(())
}

pub fn accel(mpu: &mut I2C<I2C0>) -> (f32, f32, f32) {
    let mut accel_buf = [0; 6];

    mpu.write(SLAVE_ADDR, &[ACCEL]).unwrap();
    mpu.read(SLAVE_ADDR, &mut accel_buf).unwrap();

    (concat(&accel_buf[0..2]) / 16384.0, concat(&accel_buf[2..4]) / 16384.0, concat(&accel_buf[4..6]) / 16384.0)
}

pub fn gyro(mpu: &mut I2C<I2C0>) -> (f32, f32, f32) {
    let mut gyro_buf = [0; 6];

    mpu.write(SLAVE_ADDR, &[GYRO]).unwrap();
    mpu.read(SLAVE_ADDR, &mut gyro_buf).unwrap();

    (concat(&gyro_buf[0..2]) / 131.0, concat(&gyro_buf[2..4]) / 131.0, concat(&gyro_buf[4..6]) / 131.0)
}

pub fn temp(mpu: &mut I2C<I2C0>) -> f32 {
    let mut temp_buf = [0; 2];

    mpu.write(SLAVE_ADDR, &[TEMP]).unwrap();
    mpu.read(SLAVE_ADDR, &mut temp_buf).unwrap();

    concat(&temp_buf) / 326.8 + 25.0
}

fn concat(arr: &[u8]) -> f32 {
    (((arr[0] as u16) << 8 | arr[1] as u16) as i16) as f32
}