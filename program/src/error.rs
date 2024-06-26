use solana_program::program_error::ProgramError;

use thiserror::Error;

#[derive(Error,Debug,Copy,Clone)]
pub enum ElectionError{
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Voter Already Registered")]
    VoterAlreadyRegistered,
    #[error("Voter Not Registered")]
    VoterNotRegistered,
    #[error("Invalid Candidate Index")]
    InvalidCandidateIndex,
}

impl From<ElectionError> for ProgramError{
    fn from(e:ElectionError)->Self{
        ProgramError::Custom(e as u32)
    }
}