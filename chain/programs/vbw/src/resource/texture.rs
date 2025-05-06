use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    //SOLANA_PDA_LEN,
    ResourceMap,
    VBW_SEEDS_RESOURE_MAP
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///Add new IPFS texture
pub fn texture_add(
    _ctx: Context<AddTexture>,      //default from system
    _ipfs:String,                    //IPFS cid
) -> Result<()> {

    //1.check input
    //1.1. check ipfs format
    //1.2. check wether IPFS file is exsisted    

    Ok(())
}

pub fn texture_approve(
    _ctx: Context<ApproveTexture>, 
    _index:u64,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn texture_complain(
    _ctx: Context<ComplainTexture>, //default from system
    _json:String,                     //complain JSON string
    _index:u64,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn texture_recover(
    _ctx: Context<RecoverTexture>,      //default from system
    _index:u64,                    //texture index in queue
) -> Result<()> {
    
    Ok(())
}


/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

// fn is_manage_account() -> bool{
//     return true;
// }


/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/

#[derive(Accounts)]
pub struct AddTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_RESOURE_MAP],bump)]
    pub resource_map: Account<'info, ResourceMap>,  
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