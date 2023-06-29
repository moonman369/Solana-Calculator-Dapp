use anchor_lang::prelude::*;

declare_id!("721Nyj83Zm47fcmPwJXLBC5bvGiDYxCdNCLmgMmLec5E");

#[program]
pub mod slcalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
