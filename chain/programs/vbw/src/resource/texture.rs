use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///!important, init the VBW system.

pub fn init(
    _ctx: Context<InitVBW>,      //default from system
) -> Result<()> {

    Ok(())
}

/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

fn is_valid_name() -> bool{
    return true;
}


/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/

#[derive(Accounts)]
pub struct InitVBW<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("System is inited already.")]
    AlreadyInited,
}