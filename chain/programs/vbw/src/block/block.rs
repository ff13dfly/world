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

pub fn mint(
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

pub fn update(
    _ctx: Context<UpdateBlock>,      //default from system
    account:String,                //Account storage the block data
    x:u32,                      
    y:u32,
    world:u32,
)-> Result<()> {

    //1. input check

    Ok(())
}


pub fn sell(
    _ctx: Context<SellBlock>,      //default from system
    price:u32,                      //Selling price in SOL
    x:u32,                      
    y:u32,
    world:u32,
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

pub fn complain(
    _ctx: Context<ComplainBlock>,      //default from system
    data:String,                     //complain JSON string
    x:u32,                      
    y:u32,
    world:u32,
) -> Result<()> {

    //1. input check

    Ok(())
}

pub fn recover(
    _ctx: Context<RecoverBlock>,      //default from system
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
pub struct UpdateBlock<'info> {
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

#[derive(Accounts)]
pub struct ComplainBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RecoverBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Block is inited already.")]
    AlreadyInited,
}