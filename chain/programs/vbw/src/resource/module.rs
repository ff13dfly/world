use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    ResourceMap,
    ModuleData,
    ComplainData,
    VBW_SEEDS_RESOURE_MAP,
    VBW_SEEDS_MODULE_DATA,
    VBW_SEEDS_COMPLAIN_DATA,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///!important, init the VBW system.

pub fn module_add(
    _ctx: Context<AddModule>,      //default from system
    _ipfs:String,                    //IPFS cid
    _index:u32,
) -> Result<()> {

    Ok(())
}


pub fn module_approve(
    _ctx: Context<ApproveModule>,      //default from system                   
    _index:u32,
) -> Result<()> {

    Ok(())
}

pub fn module_complain(
    _ctx: Context<ComplainModule>,      //default from system
    _json:String,                     //complain JSON string                 
    _index:u32,
) -> Result<()> {

    Ok(())
}

pub fn module_recover(
    _ctx: Context<RecoverModule>,      //default from system                   
    _index:u32,
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
#[instruction(ipfs:String,index:u32)]
pub struct AddModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_RESOURE_MAP],bump)]
    pub resource_map: Account<'info, ResourceMap>,

    #[account(
        init,
        space = SOLANA_PDA_LEN + ModuleData::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_MODULE_DATA,
            &index.to_le_bytes(),
        ],
        bump,
    )]
    pub module_data: Account<'info, ModuleData>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct ApproveModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_MODULE_DATA,&index.to_le_bytes()],bump)]
    pub module_data: Account<'info, ModuleData>,
}


#[derive(Accounts)]
#[instruction(json:String,index:u32)]
pub struct ComplainModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = SOLANA_PDA_LEN + ComplainData::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_MODULE_DATA,      //need to set [u8;4] to avoid error
            &index.to_le_bytes(),
        ],
        bump,
    )]
    pub complain_data: Account<'info, ComplainData>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct RecoverModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_MODULE_DATA,&index.to_le_bytes()],bump)]
    pub module_data: Account<'info, ModuleData>,
}