use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    HoldingAccount,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///!important, init the VBW system.

pub fn init(
    _ctx: Context<MintBlock>,      //default from system
    x:u32,                      
    y:u32,
    world:u32,
) -> Result<()> {

    //1. input check

    //2. logical check

    //3. init block

    Ok(())
}


pub fn sell(
    _ctx: Context<SellBlock>,      //default from system
    x:u32,                      
    y:u32,
    world:u32,
    price:u32,                      //Selling price in SOL
) -> Result<()> {

    //1. input check

    Ok(())
}

pub fn buy(
    _ctx: Context<BuyBlock>,      //default from system
    x:u32,                      
    y:u32,
    world:u32,
) -> Result<()> {

    //1. input check

    Ok(())
}


pub fn revoke(
    _ctx: Context<RevokeBlock>,      //default from system
    x:u32,                      
    y:u32,
    world:u32,
) -> Result<()> {

    //1. input check

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
pub struct MintBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SellBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Block is inited already.")]
    AlreadyInited,
}