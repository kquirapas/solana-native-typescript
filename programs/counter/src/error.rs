use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive)]
pub enum CounterError {
    #[error("account already initialized")]
    AlreadyInitialized,
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        // https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for CounterError {
    fn type_of() -> &'static str {
        "CounterError"
    }
}

impl PrintProgramError for CounterError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            CounterError::AlreadyInitialized => msg!("Error: account already initialized"),
        }
    }
}
