pub trait NullTerminatedStr {
    fn null_terminated_string(&self) -> String;
}
impl NullTerminatedStr for [u8] {
    fn null_terminated_string(&self) -> String {
        let mut v = vec![];
        for x in self {
            if *x == 0 {
                break;
            }
            v.push(*x);
        }
        String::from_utf8_lossy(&v).to_string()
    }
}