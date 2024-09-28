pub trait ToNBytes {
    fn n_bytes<const N: usize>(&self) -> [u8; N];
}
impl ToNBytes for [u8] {
    fn n_bytes<const N: usize>(&self) -> [u8; N] {
        let src = self;
        let mut bytes = [0u8; N];
        for idx in 0..src.len().min(N) {
            bytes[idx] = src[idx];
        }
        bytes
    }
}
impl ToNBytes for str {
    fn n_bytes<const N: usize>(&self) -> [u8; N] {
        self.as_bytes().n_bytes()
    }
}
