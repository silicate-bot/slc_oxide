pub trait Meta {
    fn size() -> u64;

    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Box<[u8]>;
}

impl Meta for () {
    fn size() -> u64 {
        0
    }

    fn from_bytes(_bytes: &[u8]) -> Self {}
    fn to_bytes(&self) -> Box<[u8]> {
        Box::new([])
    }
}
