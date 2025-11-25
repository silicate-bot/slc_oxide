use std::io::{Read, Write};

pub const METADATA_SIZE: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    pub tps: f64,
    pub seed: u64,
    pub version: u32,
    pub build: u32,
    padding: [u8; 40],
}

impl Metadata {
    pub fn new(tps: f64, seed: u64, build: u32) -> Self {
        Self {
            tps,
            seed,
            version: 1,
            build,
            padding: [0; 40],
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        let tps = f64::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let seed = u64::from_le_bytes(buf);

        let mut buf4 = [0u8; 4];
        reader.read_exact(&mut buf4)?;
        let version = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4)?;
        let build = u32::from_le_bytes(buf4);

        let mut padding = [0u8; 40];
        reader.read_exact(&mut padding)?;

        Ok(Self {
            tps,
            seed,
            version,
            build,
            padding,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.tps.to_le_bytes())?;
        writer.write_all(&self.seed.to_le_bytes())?;
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.build.to_le_bytes())?;
        writer.write_all(&self.padding)?;
        Ok(())
    }
}
