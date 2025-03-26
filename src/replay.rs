use std::io::Read;

use thiserror::Error;

use crate::meta::Meta;

pub struct Replay<M: Meta> {
    pub fps: f64,
    pub meta: M,
}

#[derive(Debug, Error)]
pub enum ReplayError {
    #[error("Header mismatch error")]
    HeaderMismatchError,
    #[error("Meta size mismatch error")]
    MetaSizeMismatchError,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Blob error: {0}")]
    Blob(#[from] crate::blob::BlobError),
}

impl<M: Meta> Replay<M> {
    const HEADER: [u8; 4] = [0x53, 0x49, 0x4C, 0x4C]; // SILL
    const FOOTER: [u8; 3] = [0x45, 0x4F, 0x4D]; // EOM

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, ReplayError> {
        let mut header_buf = [0u8; 4];
        reader.read_exact(&mut header_buf)?;

        if header_buf != Self::HEADER {
            return Err(ReplayError::HeaderMismatchError);
        }

        let mut big_buf = [0u8; 8];
        reader.read_exact(&mut big_buf)?;
        let tps = f64::from_le_bytes(big_buf);

        reader.read_exact(&mut big_buf)?;
        let meta_size = u64::from_le_bytes(big_buf);
        if meta_size != M::size() {
            return Err(ReplayError::MetaSizeMismatchError);
        }

        todo!()
    }
}
