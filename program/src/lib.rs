use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("--- instruction_data: {:?}", instruction_data);
    msg!("--- accounts.len {}", accounts.len());

    let accounts_iter = &mut accounts.iter();
    let from = next_account_info(accounts_iter)?;
    let to = next_account_info(accounts_iter)?;

    let lamports = instruction_data[0] as u64;
    msg!("--- Sending {} lamports from {} to {}...", lamports, from.key, to.key);
    **from.try_borrow_mut_lamports()? -= lamports;
    **to.try_borrow_mut_lamports()? += lamports;

    Ok(())
}