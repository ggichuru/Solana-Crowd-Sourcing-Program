use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar
};


  // ------------------------------//
 //          ENTRY POINT          //
//-------------------------------//

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // Makes sure there's instruction data
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    
    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()]
        )
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()]
        )
    } else if instruction_data[0] == 2 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()]
        )
    }

    // Throw error if instruction_data doesn't match
    msg!("Didn't find the required entrypoint");
    Err(ProgramError::InvalidInstructionData)
} 


entrypoint!(process_instruction);




  // ------------------------------//
 //          CREATE CAMPAIGN      //
//-------------------------------//

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]) -> ProgramResult {
        Ok(())
    }



  // ------------------------------//
 //          DONATE               //
//-------------------------------//

fn donate(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]) -> ProgramResult {
            Ok(())
}



  // ------------------------------//
 //          WITHDRAW             //
//-------------------------------//

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]) -> ProgramResult {
        Ok(())
}
