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

use borsh::{BorshDeserialize, BorshSerialize};

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
 //         CREATE CAMPAIGN  (0)  //
//-------------------------------//

// Campaign Details struct
#[derive(BorshDeserialize, BorshSerialize)]
struct CampaignDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub amount_donated: u64,
}


// Create Campaign function
fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]) -> ProgramResult {
        // Create an iterator on accounts
        let accounts_iter = &mut accounts.iter();

        // our writing account (program_account)
        let writing_account = next_account_info(accounts_iter)?;

        // account of the person creating the campaign
        let creater_account = next_account_info(accounts_iter)?;

        // Make sure the account signing this tx is the creator_account
        if !creater_account.is_signer {
            msg!("Creator Account should be the signer");
            return Err(ProgramError::IncorrectProgramId);
        }

        //Make sure this account is owned by the program
        if writing_account.owner != program_id {
            msg!("Writting Account should be owned by program");
            return Err(ProgramError::IncorrectProgramId);
        }

        
        // Get input_data
        let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
            .expect("Failed to deserialize input data");
        
        
        // Make sure only acdmin created a campaign
        if input_data.admin != *creater_account.key {
            msg!("Invalid instruction data");
            return Err(ProgramError::InvalidInstructionData);
        }


        // Get the minimum balance we need in out program account
        let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());

        // Make sure the account has enough balance(lamports) to create a campaign
        if **writing_account.lamports.borrow() < rent_exemption {
            msg!("Not enough balance. Should be more than the rent_exempt");
            return Err(ProgramError::InsufficientFunds);
        }


        // Set the initial amount donated to 0
        input_data.amount_donated = 0;


        // Serialize the input_data to write data in a writing account (program account)
        input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;

        Ok(())
    }


  // ------------------------------//
 //          WITHDRAW (1)         //
//-------------------------------//

// Struct to hold the amount to withdraw
#[derive(BorshDeserialize, BorshSerialize)]
struct WithdrawRequest {
    pub amount: u64,
}

// Withdraw function
fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]) -> ProgramResult {

        let accounts_iter = &mut accounts.iter();
        let writing_account = next_account_info(accounts_iter)?;
        let admin_account = next_account_info(accounts_iter)?;

        // Check if writing_account is owned by the program
        if writing_account.owner != program_id {
            msg!("Writting Account should be owned by program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // Admin account should be the signer of this tx
        if !admin_account.is_signer {
            msg!("Admin Account should be the signer");
            return Err(ProgramError::IncorrectProgramId);
        }

        // Get campaign data and check if admin's_a/c pub key is the same as the one stored in campaign data
        let campaign_data = CampaignDetails::try_from_slice(*writing_account.data.borrow())
            .expect("Error desirializing campaign data");
        
        if campaign_data.admin != *admin_account.key {
            msg!("Only Admin Account can withdraw");
            return Err(ProgramError::InvalidAccountData);
        }


        // Get input_data to get the amount of lamport the admin wants to withdraw
        let input_data = WithdrawRequest::try_from_slice(&instruction_data)
            .expect("Instruction data deserialization failed");
        
        // To prevent campaign from being deleted after a withdrawal, always have a minimum balance
        
        let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());

        // Check if lamports are enough
        if **writing_account.lamports.borrow() - rent_exemption < input_data.amount {
            msg!("Insufficient Funds.");
            return Err(ProgramError::InsufficientFunds);
        }


        /* 
            * Transfer Balance 
            * HOW??
            * Decrease the balance of the program_account and increase the admina_account balance
        */

        **writing_account.try_borrow_mut_lamports()? -= input_data.amount;
        **admin_account.try_borrow_mut_lamports()? += input_data.amount;

        Ok(())
}



  // ------------------------------//
 //          DONATE (2)           //
//-------------------------------//

fn donate(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]) -> ProgramResult {
            Ok(())
}


