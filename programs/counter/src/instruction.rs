use borsh::{BorshDeserialize, BorshSerialize};

// Instruction List
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstructionCode {
    InitializeCounter(u8), // canonical bump: u8
    IncrementCounter,      // + 1
    DecrementCounter,      // - 1
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterInstruction {
    pub code: CounterInstructionCode,
}
