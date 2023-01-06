pub fn make_crc_table() -> [u32; 256] {
    let mut table: [u32; 256] = [0; 256];
    for n in 0..256 {
        let mut c = n;
        for _ in 0..8 {
            if c & 1 == 1 {
                c = 0xedb88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        table[n] = c as u32;
    }
    table
}

pub fn update_crc(crc: u32, buf: &[u8]) -> u32 {
    let table = make_crc_table();
    let mut c = crc;
    for &b in buf {
        c = table[(c ^ b as u32) as usize & 0xff] ^ (c >> 8);
    }
    c
}

pub fn crc(buf: &[u8]) -> u32 {
    update_crc(0xffffffff, buf) ^ 0xffffffff
}
