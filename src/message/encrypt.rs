const KEY: &[u8] = "taomee_seer2_k_~#t".as_bytes();
const NO_ENCRYPT_LEN: usize = 6;

pub fn encrypt(bytes: &[u8]) -> Vec<u8> {
    let mut out_bytes = Vec::new();
    out_bytes.resize(bytes.len() + 1, 0);

    let mut i = 0;
    let mut j = NO_ENCRYPT_LEN;
    let len = bytes.len();
    while j < len {
        if i == KEY.len() {
            i = 0;
        }
        out_bytes[j] = KEY[i] ^ bytes[j];
        i += 1;
        j += 1;
    }
    let mut j = out_bytes.len() - 1;
    while j > NO_ENCRYPT_LEN {
        out_bytes[j] = out_bytes[j] | ((out_bytes[j - 1] & 0xff) >> 3);
        out_bytes[j - 1] <<= 5;
        j -= 1;
    }
    out_bytes[NO_ENCRYPT_LEN] |= 3;
    write_header(bytes, &mut out_bytes);
    out_bytes
}

pub fn decrypt(bytes: &[u8]) -> Vec<u8> {
    let mut out_bytes = Vec::new();
    out_bytes.resize(bytes.len() - 1, 0);
    let mut i = 0;
    let mut j = NO_ENCRYPT_LEN;
    let len = out_bytes.len();

    while j < len {
        if i == KEY.len() {
            i = 0;
        }
        out_bytes[j] = ((bytes[j] & 0xff) >> 5) | (bytes[j + 1] << 3);
        out_bytes[j] ^= KEY[i];
        i += 1;
        j += 1;
    }
    write_header(bytes, &mut out_bytes);
    out_bytes
}

fn write_header(src: &[u8], tar: &mut Vec<u8>) {
    let len = tar.len();
    tar[0] = (len & 0xff) as u8;
    tar[1] = ((len >> 8) & 0xff) as u8;
    tar[2] = ((len >> 16) & 0xff) as u8;
    tar[3] = ((len >> 24) & 0xff) as u8;
    tar[4] = src[4];
    tar[5] = src[5];
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn test() {
        let raw = "1231241345675678".as_bytes();
        let r = encrypt(raw);
        let d = decrypt(r.as_slice());
        assert_eq!(&raw[4..], &d.as_slice()[4..])
    }
}
