use hidapi::{DeviceInfo, HidApi};

use crate::{
    decrypt,
    prometheus::{CO2, TEMP},
};

fn find(ctx: &HidApi) -> &DeviceInfo {
    for dev in ctx.device_list() {
        if dev.vendor_id() == 0x04d9 && dev.product_id() == 0xa052 {
            return dev;
        }
    }
    panic!("Device not found");
}

pub fn read_hid() -> ! {
    let api = HidApi::new().unwrap();
    let dev = find(&api);
    let dev = dev.open_device(&api).unwrap();
    dev.send_feature_report(&{
        let mut arr = [0; 9];
        arr[1..].copy_from_slice(decrypt::KEY);
        arr
    })
    .unwrap();
    loop {
        let mut buf = [0; 8];
        dev.read(&mut buf).unwrap();
        decrypt::decrypt(&mut buf);
        if buf[4] != 0x0d || buf[..3].iter().map(|x| *x as u32).sum::<u32>() as u8 != buf[3] {
            log::warn!("Invalid checksum");
            continue;
        }
        let op = buf[0];
        let val = u16::from_be_bytes([buf[1], buf[2]]);
        match op {
            0x50 => CO2.set(val as _),
            0x42 => TEMP.set((val as f64) / 16.0 - 273.15),
            _ => log::debug!("Unknown op: {op:?} with value {val:?}"),
        }
    }
}
