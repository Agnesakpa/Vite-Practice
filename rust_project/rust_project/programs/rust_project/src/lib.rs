use anchor_lang::prelude::*;

declare_id!("83QsaV9YXKQGgJevcSuyUWLgJp3x87q2AeBTw35mTg1d");

#[program]
pub mod rust_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
