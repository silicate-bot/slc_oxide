use std::io::{Read, Seek, Write};
use thiserror::Error;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomId {
    Null = 0,
    Action = 1,
    Marker = 2,
}

impl TryFrom<u32> for AtomId {
    type Error = AtomError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AtomId::Null),
            1 => Ok(AtomId::Action),
            2 => Ok(AtomId::Marker),
            _ => Err(AtomError::UnknownAtomId(value)),
        }
    }
}

#[derive(Debug, Error)]
pub enum AtomError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Unknown atom ID: {0}")]
    UnknownAtomId(u32),
    #[error("Atom body is too large")]
    AtomTooLarge,
    #[error("Section error: {0}")]
    SectionError(#[from] crate::v3::section::SectionError),
}

pub trait Atom: Sized {
    const ID: AtomId;

    fn size(&self) -> usize;
    fn read<R: Read>(reader: &mut R, size: usize) -> Result<Self, AtomError>;
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), AtomError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueAtom {
    pub id: u32,
    pub flags: u8,
    pub body: Vec<u8>,
}

impl OpaqueAtom {
    pub fn new(id: u32, flags: u8, body: Vec<u8>) -> Self {
        Self { id, flags, body }
    }
}

pub enum AtomVariant {
    Opaque(OpaqueAtom),
    Action(super::builtin::ActionAtom),
}

impl AtomVariant {
    pub fn id(&self) -> u32 {
        match self {
            AtomVariant::Opaque(a) => a.id,
            AtomVariant::Action(_) => AtomId::Action as u32,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            AtomVariant::Opaque(a) => a.body.len(),
            AtomVariant::Action(a) => a.size(),
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, AtomError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let id = u32::from_le_bytes(buf);

        let mut buf8 = [0u8; 8];
        reader.read_exact(&mut buf8)?;
        let size_and_flags = u64::from_le_bytes(buf8);
        let flags = (size_and_flags >> 56) as u8;
        let size = usize::try_from(size_and_flags & 0x00ff_ffff_ffff_ffff)
            .map_err(|_| AtomError::AtomTooLarge)?;

        match id {
            id if id == AtomId::Action as u32 => Ok(AtomVariant::Action(
                super::builtin::ActionAtom::read(reader, size)?,
            )),
            _ => {
                let mut body = vec![0u8; size];
                reader.read_exact(&mut body)?;
                Ok(AtomVariant::Opaque(OpaqueAtom::new(id, flags, body)))
            }
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), AtomError> {
        let mut body = Vec::new();
        let flags = match self {
            AtomVariant::Opaque(a) => {
                body.extend_from_slice(&a.body);
                a.flags
            }
            AtomVariant::Action(a) => {
                a.write(&mut body)?;
                0
            }
        };

        let id = self.id();
        writer.write_all(&id.to_le_bytes())?;
        let size = u64::try_from(body.len()).map_err(|_| AtomError::AtomTooLarge)?;
        if size > 0x00ff_ffff_ffff_ffff {
            return Err(AtomError::AtomTooLarge);
        }
        let size_and_flags = size | (u64::from(flags) << 56);
        writer.write_all(&size_and_flags.to_le_bytes())?;
        writer.write_all(&body)?;

        Ok(())
    }
}

pub struct AtomRegistry {
    pub atoms: Vec<AtomVariant>,
}

impl AtomRegistry {
    pub fn new() -> Self {
        Self { atoms: Vec::new() }
    }

    pub fn add(&mut self, atom: AtomVariant) {
        self.atoms.push(atom);
    }

    pub fn read_all<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        end_pos: u64,
    ) -> Result<(), AtomError> {
        loop {
            let current_pos = reader.stream_position()?;
            if current_pos >= end_pos {
                break;
            }
            let atom = AtomVariant::read(reader)?;
            self.add(atom);
        }
        Ok(())
    }

    pub fn write_all<W: Write>(&self, writer: &mut W) -> Result<(), AtomError> {
        for atom in &self.atoms {
            atom.write(writer)?;
        }
        Ok(())
    }
}

impl Default for AtomRegistry {
    fn default() -> Self {
        Self::new()
    }
}
