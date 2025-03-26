use std::io::{Read, Write};

use thiserror::Error;

pub struct PlayerInput {
    pub hold: bool,
    pub player_2: bool,
    pub button: u8,
}

pub enum InputData {
    Skip,
    Player(PlayerInput),
    Restart,
    RestartFull,
    Death,
    TPS(f64),
}

pub struct Input {
    delta: u64,
    pub frame: u64,
    pub data: InputData,
}

// IO

#[derive(Debug, Error)]
pub enum InputError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Invalid TPS provided")]
    InvalidTPS,
    #[error("Invalid button type")]
    InvalidButton,
}

impl Input {
    pub fn read<R: Read>(
        reader: &mut R,
        current_frame: u64,
        byte_size: usize,
    ) -> Result<Self, InputError> {
        let mut buf = vec![0u8; byte_size];
        reader.read_exact(&mut buf)?;
        buf.resize(8, 0);

        let state = u64::from_le_bytes(*unsafe {
            std::mem::transmute::<*const u8, &[u8; 8]>(buf.as_ptr())
        });

        let delta = state >> 5;
        let frame = current_frame + delta;
        let button = (state & 0b11100) >> 2;

        let data = match button {
            0 => InputData::Skip,
            1..=3 => InputData::Player(PlayerInput {
                hold: (state & 1) != 0,
                player_2: (state & 2) != 0,
                button: button as u8,
            }),
            4 => InputData::Restart,
            5 => InputData::RestartFull,
            6 => InputData::Death,
            7 => {
                reader.read_exact(&mut buf)?;
                let tps = f64::from_le_bytes(*unsafe {
                    std::mem::transmute::<*const u8, &[u8; 8]>(buf.as_ptr())
                });

                InputData::TPS(tps)
            }
            _ => return Err(InputError::InvalidButton),
        };

        Ok(Input { delta, frame, data })
    }

    const fn to_state(&self) -> u64 {
        let state: u64 = match self.data {
            InputData::Skip => 0 << 2,
            InputData::Player(PlayerInput {
                button,
                hold,
                player_2,
            }) => ((button as u64) << 2) | hold as u64 | ((player_2 as u64) << 1),
            InputData::Restart => 4 << 2,
            InputData::RestartFull => 5 << 2,
            InputData::Death => 6 << 2,
            InputData::TPS(_) => 7 << 2,
        };

        state | (self.delta << 5)
    }

    pub fn write<W: Write>(&self, writer: &mut W, byte_size: u64) -> Result<(), InputError> {
        writer.write_all(&self.to_state().to_le_bytes()[0..byte_size as usize])?;
        if let InputData::TPS(tps) = self.data {
            writer.write_all(&tps.to_le_bytes())?;
        }

        Ok(())
    }
}
