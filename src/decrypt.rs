pub const KEY: &[u8; 8] = b"\xc4\xc6\xc0\x92\x40\x23\xdc\x96";

pub fn decrypt(data: &mut [u8; 8]) {
    const CSTATE: &[u8; 8] = b"\x48\x74\x65\x6D\x70\x39\x39\x65";
    const SHUFFLE: &[usize; 8] = &[2, 4, 0, 7, 1, 6, 5, 3];

    let mut buf = [0; 8];
    for (i, o) in SHUFFLE.iter().enumerate() {
        buf[*o] = data[i] ^ KEY[*o];
    }

    for i in 0..8 {
        data[i] = ((buf[i] >> 3) | (buf[(i + 8 - 1) % 8] << 5))
            .wrapping_sub((CSTATE[i] >> 4) | (CSTATE[i] << 4));
    }
}
