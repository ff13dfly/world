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

///Add new IPFS texture
pub fn add(
    _ctx: Context<AddTexture>,      //default from system
    ipfs:String,                    //IPFS cid
) -> Result<()> {
    
    Ok(())
}

pub fn approve(
    _ctx: Context<ApproveTexture>, 
    index:u64,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn complain(
    _ctx: Context<ComplainTexture>, //default from system
    index:u64,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn recover(
    _ctx: Context<RecoverTexture>,      //default from system
    index:u64,                    //texture index in queue
) -> Result<()> {
    
    Ok(())
}


/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

fn is_manage_account() -> bool{
    return true;
}


/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/

#[derive(Accounts)]
pub struct AddTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct ComplainTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RecoverTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("System is inited already.")]
    AlreadyInited,
}