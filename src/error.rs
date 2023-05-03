use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum HeroError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Hero fully added.
    #[error("Hero overflow")]
    HeroOverflow,
    /// Hero fully added.
    #[error("Invalid NFT Key")]
    InvalidNFTKey,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<HeroError> for ProgramError {
    fn from(e: HeroError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
