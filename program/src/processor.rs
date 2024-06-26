use crate::{error::ElectionError, instruction::ElectionInstruction, state::Election};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize,BorshSerialize};


pub struct Processor;

impl Processor {
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = ElectionInstruction::unpack(instruction_data)?;

        match instruction {
            ElectionInstruction::CreateElection { name, candidates } => {
                Self::process_create_election(accounts, name, candidates)
            }
            ElectionInstruction::RegisterVoter { voter_pubkey } => {
                Self::process_register_voter(accounts, voter_pubkey)
            }
            ElectionInstruction::CastVote { voter_pubkey, candidate_index } => {
                Self::process_cast_vote(accounts, voter_pubkey, candidate_index)
            }
        }
    }

    fn process_create_election(
        accounts: &[AccountInfo],
        name: String,
        candidates: Vec<String>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let election_account = next_account_info(account_info_iter)?;

        let mut election = Election::try_from_slice(&election_account.data.borrow())?;
        election.name = name;
        election.candidates = candidates;
        election.votes = vec![0; election.candidates.len()];
        election.voters = Vec::new();

        election.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_register_voter(accounts: &[AccountInfo], voter_pubkey: Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let election_account = next_account_info(account_info_iter)?;

        let mut election = Election::try_from_slice(&election_account.data.borrow())?;
        if election.voters.contains(&voter_pubkey) {
            msg!("Voter is already registered.");
            return Err(ElectionError::VoterAlreadyRegistered.into());
        }

        election.voters.push(voter_pubkey);
        election.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_cast_vote(accounts: &[AccountInfo], voter_pubkey: Pubkey, candidate_index: u8) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let election_account = next_account_info(account_info_iter)?;

        let mut election = Election::try_from_slice(&election_account.data.borrow())?;
        if !election.voters.contains(&voter_pubkey) {
            msg!("Voter is not registered.");
            return Err(ElectionError::VoterNotRegistered.into());
        }

        if (candidate_index as usize) >= election.candidates.len() {
            msg!("Invalid candidate index.");
            return Err(ElectionError::InvalidCandidateIndex.into());
        }

        election.votes[candidate_index as usize] += 1;
        election.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
