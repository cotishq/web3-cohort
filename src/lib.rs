use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info , AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};
#[derive(BorshDeserialize,BorshSerialize)]
enum Instruction_type{
    Increment(u32),
    Decrement(u32)
}
#[derive(BorshDeserialize,BorshSerialize)]
struct counter{
    count : u32
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data : &[u8],
) -> ProgramResult{
    let account = next_account_info(&mut accounts.iter())?;
    let mut counter = counter::try_from_slice(&account.data.borrow())?;

    match Instruction_type::try_from_slice(instruction_data)? {

        Instruction_type::Increment(amount) => {
            counter.count += amount;
        }

        Instruction_type::Decrement(amount)=> {
            counter.count -= amount;
        }

        
    }
    counter.serialize(&mut *account.data.borrow_mut())?;

    msg!("counter updated to {}" , counter.count);

    Ok(())
}