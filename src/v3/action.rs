#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Reserved = 0,
    Jump = 1,
    Left = 2,
    Right = 3,
    Restart = 4,
    RestartFull = 5,
    Death = 6,
    TPS = 7,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub frame: u64,
    pub action_type: ActionType,
    pub holding: bool,
    pub player2: bool,
    pub seed: u64,
    pub tps: f64,
    pub(crate) swift: bool,
    delta: u64,
}

impl Action {
    pub fn player(
        current_frame: u64,
        delta: u64,
        action_type: ActionType,
        holding: bool,
        player2: bool,
    ) -> Self {
        Self {
            frame: current_frame + delta,
            action_type,
            holding,
            player2,
            seed: 0,
            tps: 240.0,
            swift: false,
            delta,
        }
    }

    pub fn death(current_frame: u64, delta: u64, action_type: ActionType, seed: u64) -> Self {
        Self {
            frame: current_frame + delta,
            action_type,
            holding: false,
            player2: false,
            seed,
            tps: 240.0,
            swift: false,
            delta,
        }
    }

    pub fn tps_change(current_frame: u64, delta: u64, tps: f64) -> Self {
        Self {
            frame: current_frame + delta,
            action_type: ActionType::TPS,
            holding: false,
            player2: false,
            seed: 0,
            tps,
            swift: false,
            delta,
        }
    }

    pub const fn is_player(&self) -> bool {
        matches!(
            self.action_type,
            ActionType::Jump | ActionType::Left | ActionType::Right
        )
    }

    pub const fn delta(&self) -> u64 {
        self.delta
    }

    pub const fn swift(&self) -> bool {
        self.swift
    }

    pub fn recalculate_delta(&mut self, previous_frame: u64) {
        self.delta = self.frame - previous_frame;
    }

    pub const fn minimum_size(&self) -> u8 {
        let offset = if self.is_player() { 4 } else { 8 };
        let delta = self.delta;

        let one_byte_threshold = 1u64 << offset;
        let two_bytes_threshold = 1u64 << (offset + 8);
        let four_bytes_threshold = 1u64 << (offset + 24);

        if delta < one_byte_threshold {
            0
        } else if delta < two_bytes_threshold {
            1
        } else if delta < four_bytes_threshold {
            2
        } else {
            3
        }
    }
}
