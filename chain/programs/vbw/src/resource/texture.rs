use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    ResourceMap,
    TextureData,
    ComplainData,
    VBW_SEEDS_RESOURE_MAP,
    VBW_SEEDS_TEXTURE_DATA,
    VBW_SEEDS_COMPLAIN_DATA,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///Add new IPFS texture
pub fn texture_add(
    _ctx: Context<AddTexture>,      //default from system
    _ipfs:String,                    //IPFS cid
    _index:u32,                     //texture id
) -> Result<()> {

    //1.check input
    //1.1. check ipfs format
    //1.2. check wether IPFS file is exsisted    
    //1.3. check wether index valid

    Ok(())
}

pub fn texture_approve(
    _ctx: Context<ApproveTexture>, 
    _index:u32,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn texture_complain(
    _ctx: Context<ComplainTexture>, //default from system
    _json:String,                     //complain JSON string
    _index:u32,                      //texture index in queue
) -> Result<()> {
    
    Ok(())
}

pub fn texture_recover(
    _ctx: Context<RecoverTexture>,      //default from system
    _index:u32,                    //texture index in queue
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
#[instruction(ipfs:String,index:u32)]
pub struct AddTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_RESOURE_MAP],bump)]
    pub resource_map: Account<'info, ResourceMap>,  

    #[account(
        init,
        space = SOLANA_PDA_LEN + TextureData::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_TEXTURE_DATA,
            &index.to_le_bytes(),
        ],
        bump,
    )]
    pub texture_data: Account<'info, TextureData>,

    pub system_program: Program<'info, System>,
}   

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct ApproveTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub module_data: Account<'info, TextureData>,
}


#[derive(Accounts)]
#[instruction(json:String,index:u32)]
pub struct ComplainTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub module_data: Account<'info, TextureData>,

    #[account(
        init,
        space = SOLANA_PDA_LEN + ComplainData::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_COMPLAIN_DATA,      //need to set [u8;4] to avoid error
            &index.to_le_bytes(),
        ],
        bump,
    )]
    pub complain_data: Account<'info, ComplainData>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct RecoverTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub module_data: Account<'info, TextureData>,
}