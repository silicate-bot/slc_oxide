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
    #[error("Section error: {0}")]
    SectionError(#[from] crate::v3::section::SectionError),
}

pub trait Atom: Sized {
    const ID: AtomId;

    fn size(&self) -> usize;
    fn read<R: Read>(reader: &mut R, size: usize) -> Result<Self, AtomError>;
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), AtomError>;
}

pub struct NullAtom {
    pub size: usize,
}

impl Atom for NullAtom {
    const ID: AtomId = AtomId::Null;

    fn size(&self) -> usize {
        self.size
    }

    fn read<R: Read>(reader: &mut R, size: usize) -> Result<Self, AtomError> {
        let mut buf = vec![0u8; size];
        reader.read_exact(&mut buf)?;
        Ok(Self { size })
    }

    fn write<W: Write>(&self, _writer: &mut W) -> Result<(), AtomError> {
        Ok(())
    }
}

pub enum AtomVariant {
    Null(NullAtom),
    Action(super::builtin::ActionAtom),
}

impl AtomVariant {
    pub fn id(&self) -> AtomId {
        match self {
            AtomVariant::Null(_) => AtomId::Null,
            AtomVariant::Action(_) => AtomId::Action,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            AtomVariant::Null(a) => a.size(),
            AtomVariant::Action(a) => a.size(),
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, AtomError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let id = u32::from_le_bytes(buf);
        let atom_id = AtomId::try_from(id)?;

        let mut buf8 = [0u8; 8];
        reader.read_exact(&mut buf8)?;
        let size = u64::from_le_bytes(buf8) as usize;

        match atom_id {
            AtomId::Null => Ok(AtomVariant::Null(NullAtom::read(reader, size)?)),
            AtomId::Action => Ok(AtomVariant::Action(super::builtin::ActionAtom::read(
                reader, size,
            )?)),
            AtomId::Marker => Ok(AtomVariant::Null(NullAtom::read(reader, size)?)),
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), AtomError> {
        let id = self.id() as u32;
        writer.write_all(&id.to_le_bytes())?;

        let size = self.size() as u64;
        writer.write_all(&size.to_le_bytes())?;

        match self {
            AtomVariant::Null(a) => a.write(writer)?,
            AtomVariant::Action(a) => a.write(writer)?,
        }

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
