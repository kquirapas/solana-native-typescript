use borsh::{BorshDeserialize, BorshSerialize};

// Counter Data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: i64, // 8 bytes
    pub bump: u8,   // 4 bytes
}

impl Counter {
    pub const LEN: usize = 8 + 4; // bytes

    pub fn from(bump: u8) -> Self {
        Self { count: 0, bump }
    }

    pub fn increment(&mut self) {
        self.count += 1
    }

    pub fn decrement(&mut self) {
        self.count -= 1
    }
}
