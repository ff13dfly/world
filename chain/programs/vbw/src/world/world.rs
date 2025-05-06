use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    VBW_WORLD_LIST_SIZE,
    VBW_RESOURE_MAP_SIZE,
    VBW_WHITELIST_MAP_SIZE,
    VBW_TEXTURE_LIST_SIZE,
    VBW_MODULE_LIST_SIZE,
    WorldList,
    WhiteList,
    ResourceMap,
    WorldCounter,
    TextureList,
    ModuleList,
    VBW_SEEDS_WORLD_LIST,
    VBW_SEEDS_RESOURE_MAP,
    VBW_SEEDS_WHITE_LIST,
    VBW_SEEDS_WORLD_COUNT,
    VBW_SEEDS_TEXTURE_LIST,
    VBW_SEEDS_MODULE_LIST,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/


pub fn init(
    _ctx: Context<InitVBW>,     //default from system
    _root:String,               //root address
) -> Result<()> {   

    //1. create world necessary accounts.
    //1.1. world list 
    //1.2. texture queue
    //1.3. module queue
    //1.4. adjunct map

    //2. create management accounts.
    //2.1. whitelist of manage for whole system. ( ban texture, ban module ) 
    //2.2. whitelist of manage for single world. ( ban block )

    Ok(())
}

pub fn start(
    _ctx: Context<NewWorld>,    //default from system
    _index:u32,                  //index of world to  start
    _setting:String,             //world setting as JSON format
) -> Result<()> {

    //0. input check
    //0.1. wether valid index.
    //0.2. wether valid setting.

    //1. logical check
    //1.1. ready to start new world.
    


    //2. create world accounts
    //2.1. world sold counter

    //3. write world setting
    //3.1. update new world setting
    //3.2. close the update of old world

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
pub struct InitVBW<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /************ PDA accounts ************/
    #[account(
        init,
        space = SOLANA_PDA_LEN + VBW_WHITELIST_MAP_SIZE, 
        payer = payer,
        seeds = [VBW_SEEDS_WHITE_LIST],
        bump,
    )]
    pub whitelist_account: Account<'info, WhiteList>,


    //FIXME, need to relocate the size of `world_list` in final version
    #[account(
        init,
        space = SOLANA_PDA_LEN + VBW_WORLD_LIST_SIZE,     
        payer = payer,
        seeds = [VBW_SEEDS_TEXTURE_LIST],
        bump,
    )]
    pub world_list: Account<'info, WorldList>, 

    //important, need to take care of `resource_map`, as the size rise up, need more account size, even overflow
    //importnat, used to check wether the IPFS link is exsisted.
    //FIXME, need to relocate the size of `resource_map` in final version
    // #[account(
    //     init,
    //     space = SOLANA_PDA_LEN + VBW_RESOURE_MAP_SIZE,     
    //     payer = payer,
    //     seeds = [VBW_SEEDS_RESOURE_MAP],
    //     bump,
    // )]
    // pub resource_map: Account<'info, ResourceMap>,


    //FIXME, counter to manage the texture
    // #[account(
    //     init,
    //     space = SOLANA_PDA_LEN + VBW_TEXTURE_LIST_SIZE,     
    //     payer = payer,
    //     seeds = [VBW_SEEDS_RESOURE_MAP],
    //     bump,
    // )]
    // pub texture_list: Account<'info, TextureList>,

    //FIXME, counter to manage the module
    // #[account(
    //     init,
    //     space = SOLANA_PDA_LEN + VBW_MODULE_LIST_SIZE,     
    //     payer = payer,
    //     seeds = [VBW_SEEDS_MODULE_LIST],
    //     bump,
    // )]
    // pub module_list: Account<'info, ModuleList>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct NewWorld<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    //WorldCounter, if needed.
    #[account(
        init,
        space = SOLANA_PDA_LEN + WorldCounter::INIT_SPACE, 
        payer = payer,
        seeds = [
            VBW_SEEDS_WORLD_COUNT,
            //index
        ],
        bump,
    )]
    pub world_counter: Account<'info, WorldCounter>,

    #[account(mut,seeds = [VBW_SEEDS_WHITE_LIST],bump)]
    pub whitelist_account: Account<'info, WhiteList>,

    #[account(mut,seeds = [VBW_SEEDS_WORLD_LIST],bump)]
    pub world_list: Account<'info, WorldList>,

    pub system_program: Program<'info, System>,
}