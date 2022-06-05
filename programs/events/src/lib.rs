use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod events {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent {
            data: 10,
            label: "event".to_string(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub data: u64,
    pub label: String,
}
