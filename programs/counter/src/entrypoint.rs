use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)?;
    Ok(())
}

// // Process Instruction Byte-Array
// let instr = Instruction::try_from_slice(instruction_data)?;
//
// // Process Accounts
// let accounts_info_iter = &mut accounts.iter();
//
// // Payer Account
// let payer_account: &AccountInfo = next_account_info(accounts_info_iter)?;
//
// // Mint Account PDA
// let mint_account: &AccountInfo = next_account_info(accounts_info_iter)?;
//
// // On-chain PDA Derivations
// // TODO: Consider using create_program_address here to save
// // compute costs and use find_program_address offchain instead
// let (mint_pda, _) =
//     Pubkey::find_program_address(&[payer_account.key.as_ref(), b"mint_account"], program_id);
//
// // SYSVAR fetch
// let rent_sysvar = Rent::get()?;
//
// //----- ASSERTIONS AND SAFETY CHECKS -----
// // Validate proper payer account permissions
// assert!(payer_account.is_signer);
// assert!(payer_account.is_writable);
// // Validate proper mint PDA account permissions
// assert!(!mint_account.is_signer);
// assert!(mint_account.is_writable);
// // Verify correctness of PDA derivation with canonical bumps
// assert_eq!(mint_pda, *mint_account.key);
// assert_eq!(mint_authority_pda, *mint_authority_account.key);
// assert_eq!(ata_pda, *ata_account.key);
// // Verify solana program accounts passed
// assert_eq!(SYSVAR_RENT_ID, *rent_account.key);
// assert_eq!(SYSTEM_PROGRAM_ID, *system_program_account.key);
// assert_eq!(TOKEN_2022_PROGRAM_ID, *token_program_account.key);
//
// // create mint account instruction
// let create_counter = &system_instruction::create_account(
//     payer_account.key,
//     mint_account.key,
//     // Getting total rent needed by mint + extensions + metadata ahead
//     // instead of using "reallocation"
//     mint_account_rent as u64,
//     mint_account_space as u64,
//     &TOKEN_2022_PROGRAM_ID,
// );
//
// // standard Solana program exit signal for success
// Ok(())
