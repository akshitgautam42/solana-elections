use borsh :: {BorshDeserialize , BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey
};

#[derive (BorshSerialize,BorshDeserialize,Debug)]
pub enum ElectionInstruction {
    CreateElection { name: String, candidates: Vec<String> },
    RegisterVoter { voter_pubkey: Pubkey },
    CastVote { voter_pubkey: Pubkey, candidate_index: u8 },
}

impl ElectionInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        match tag {
            0 => {
                let name_len = *rest.get(0).ok_or(ProgramError::InvalidInstructionData)? as usize;
                let name_end = 1 + name_len;
                if rest.len() < name_end {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let name = String::from_utf8(rest[1..name_end].to_vec()).map_err(|_| ProgramError::InvalidInstructionData)?;
                let candidates_len_index = name_end;
                let candidates_len = *rest.get(candidates_len_index).ok_or(ProgramError::InvalidInstructionData)? as usize;
                let mut candidates = Vec::new();
                let mut offset = candidates_len_index + 1;
                for _ in 0..candidates_len {
                    let candidate_len = *rest.get(offset).ok_or(ProgramError::InvalidInstructionData)? as usize;
                    let candidate_end = offset + 1 + candidate_len;
                    if rest.len() < candidate_end {
                        return Err(ProgramError::InvalidInstructionData);
                    }
                    let candidate = String::from_utf8(rest[offset + 1..candidate_end].to_vec()).map_err(|_| ProgramError::InvalidInstructionData)?;
                    candidates.push(candidate);
                    offset = candidate_end;
                }
                Ok(Self::CreateElection { name, candidates })
            },
            1 => {
                let register_voter = RegisterVoter::try_from_slice(rest)?;
                Ok(Self::RegisterVoter { voter_pubkey: register_voter.voter_pubkey })
            },
            2 => {
                let cast_vote = CastVote::try_from_slice(rest)?;
                Ok(Self::CastVote { voter_pubkey: cast_vote.voter_pubkey, candidate_index: cast_vote.candidate_index })
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RegisterVoter {
    pub voter_pubkey: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CastVote {
    pub voter_pubkey: Pubkey,
    pub candidate_index: u8,
}
