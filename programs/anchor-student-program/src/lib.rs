use anchor_lang::prelude::*;

declare_id!("B5WxNAsAxroFLJ3F5ETrSwCMMm4udpm9KnCjJ5TeX5UY");

const MAX_NAME_LENGTH: usize = 20;
const MAX_MESSAGE_LENGTH: usize = 50;

#[program]
pub mod anchor_student_program {
    use super::*;
    pub fn add_student_info(
        ctx: Context<AddStudentDetails>,
        student_name: String,
        message: String,
    ) -> Result<()> {
        //require that arguments meet certain criteria
        require!(
            student_name.len() <= MAX_NAME_LENGTH,
            ErrorMessage::NameTooLong
        );
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            ErrorMessage::MessageTooLong
        );
        msg!("Student Info Account Created");
        msg!("Name: {}", student_name);
        msg!("message: {}", message);

        //initialise PDA
        let student_info = &mut ctx.accounts.student_info;
        student_info.student = ctx.accounts.initializer.key();
        student_info.student_name = student_name;
        student_info.message = message;

        Ok(())
    }

    pub fn update_student_info(
        ctx: Context<UpdateStudentDetails>,
        student_name: String,
        message: String,
    ) -> Result<()> {
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            ErrorMessage::MessageTooLong
        );

        msg!("Student Info Account Reallocated");
        msg!("Name: {}", student_name);
        msg!("message: {}", message);

        let student_info = &mut ctx.accounts.student_info;
        student_info.student = ctx.accounts.initializer.key();
        // student_info.student_name = student_name;
        student_info.message = message;
        Ok(())
    }

    pub fn delete_student_info(
        _ctx: Context<DeleteStudentDetails>,
        student_name: String,
    ) -> Result<()> {
        msg!("Student Info Account deleted: {}", student_name);

        Ok(())
    }
}

// Initialize a PDA account for each student that stores the student's name and their short message
// Update the message on an existing account
// Close an existing account

//PDA
#[account]
#[derive(InitSpace)]
pub struct StudentAccountState {
    pub student: Pubkey,
    #[max_len(20)]
    pub student_name: String,
    #[max_len(50)]
    pub message: String,
}

const DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
#[instruction(student_name:String, message: String)]
pub struct AddStudentDetails<'info> {
    #[account(init,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer=initializer,
        space= DISCRIMINATOR + StudentAccountState::INIT_SPACE
    )]
    pub student_info: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student_name:String,message:String)]
pub struct UpdateStudentDetails<'info> {
    #[account(mut,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + StudentAccountState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true

    )]
    pub student_info: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student_name:String)]
pub struct DeleteStudentDetails<'info> {
    #[account(mut,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer

    )]
    pub student_info: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
enum ErrorMessage {
    #[msg("Student name too long")]
    NameTooLong,
    #[msg("message too long")]
    MessageTooLong,
}
