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

pub fn block(
    _ctx: Context<BanBlock>,      //default from system
    x:u32,
    y:u32,
    world:u32,
) -> Result<()> {   


    Ok(())
}

pub fn texture(
    _ctx: Context<BanTexture>,      //default from system
    index: u64,
) -> Result<()> {   


    Ok(())
}

pub fn module(
    _ctx: Context<BanModule>,      //default from system
    index: u64,
) -> Result<()> {   


    Ok(())
}



/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

// fn is_valid_name() -> bool{
//     return true;
// }


/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/

#[derive(Accounts)]
pub struct BanBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct BanTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct BanModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("System is inited already.")]
    AlreadyInited,
}