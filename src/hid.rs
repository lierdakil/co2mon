use hidapi::{DeviceInfo, HidApi};

use crate::prometheus::{CO2, TEMP};

fn find(ctx: &HidApi) -> &DeviceInfo {
    for dev in ctx.device_list() {
        if dev.vendor_id() == 0x04d9 && dev.product_id() == 0xa052 {
            return dev;
        }
    }
    panic!("Device not found");
}

pub fn decrypt(data: &mut [u8; 8]) {
    const CSTATE: &[u8; 8] = b"\x48\x74\x65\x6D\x70\x39\x39\x65";
    const SHUFFLE: &[usize; 8] = &[2, 4, 0, 7, 1, 6, 5, 3];

    let mut buf = [0; 8];
    for (i, o) in SHUFFLE.iter().enumerate() {
        buf[*o] = data[i]; // we'd xor with key[*o] here, but since it's all zeros we don't have to
    }

    for i in 0..8 {
        data[i] = ((buf[i] >> 3) | (buf[(i + 8 - 1) % 8] << 5))
            .wrapping_sub((CSTATE[i] >> 4) | (CSTATE[i] << 4));
    }
}

pub fn read_hid() -> ! {
    let api = HidApi::new().unwrap();
    let dev = find(&api);
    let dev = dev.open_device(&api).unwrap();
    // this is supposed to be an "encryption" key (8 bytes), but we send zeros
    // for simplicity.
    dev.send_feature_report(&[0; 9]).unwrap();
    loop {
        let mut buf = [0; 8];
        dev.read(&mut buf).unwrap();
        decrypt(&mut buf);
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
