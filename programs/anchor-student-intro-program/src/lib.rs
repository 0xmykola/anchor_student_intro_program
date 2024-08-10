use anchor_lang::prelude::*;
mod constants;

use constants::*;

declare_id!("7aHbT8gpfAjCvNZtkJk3dsPQJSWBGAGoAv9XK8TEodDQ");

#[program]
pub mod anchor_student_intro_program {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, message: String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), student.key().as_ref()],
        bump,
        payer = student,
        space = StudentAccountState::INIT_SPACE + name.len() + message.len())]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StudentAccountState {
    pub student: Pubkey,
    pub name: String,
    pub message: String,
}

impl Space for StudentAccountState {
    const INIT_SPACE: usize =
        ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}
