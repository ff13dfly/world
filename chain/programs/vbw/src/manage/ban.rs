use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    BlockData,
    TextureData,
    ModuleData,
    VBW_SEEDS_BLOCK_DATA,
    VBW_SEEDS_TEXTURE_DATA,
    VBW_SEEDS_MODULE_DATA,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

pub fn block(
    _ctx: Context<BanBlock>,      //default from system
    _x:u32,
    _y:u32,
    _world:u32,
) -> Result<()> {   


    Ok(())
}

pub fn texture(
    _ctx: Context<BanTexture>,      //default from system
    _index: u64,
) -> Result<()> {   


    Ok(())
}

pub fn module(
    _ctx: Context<BanModule>,      //default from system
    _index: u64,
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

    #[account(mut,seeds = [VBW_SEEDS_BLOCK_DATA],bump)]
    pub block_data: Account<'info, BlockData>,
}

#[derive(Accounts)]
pub struct BanTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA],bump)]
    pub texture_data: Account<'info, TextureData>,
}

#[derive(Accounts)]
pub struct BanModule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_MODULE_DATA],bump)]
    pub texture_data: Account<'info, ModuleData>,
}