pub trait Meta {
    fn size() -> usize;

    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Box<[u8]>;
}

impl Meta for () {
    fn size() -> usize {
        0
    }

    fn from_bytes(_bytes: &[u8]) -> Self {}
    fn to_bytes(&self) -> Box<[u8]> {
        Box::new([])
    }
}
