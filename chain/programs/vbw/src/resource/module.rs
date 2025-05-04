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

pub fn add(
    _ctx: Context<AddModule>,      //default from system
    ipfs:String,                    //IPFS cid
) -> Result<()> {

    Ok(())
}


pub fn approve(
    _ctx: Context<ApproveModule>,      //default from system                   
    index:u64,
) -> Result<()> {

    Ok(())
}

pub fn complain(
    _ctx: Context<ComplainModule>,      //default from system
    data:String,                    
    index:u64,
) -> Result<()> {

    Ok(())
}

pub fn recover(
    _ctx: Context<RecoverModule>,      //default from system
    data:String,                    
    index:u64,
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
pub struct AddModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct ComplainModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct RecoverModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("System is inited already.")]
    AlreadyInited,
}