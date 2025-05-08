use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    ResourceMap,
    TextureData,
    TextureCounter,
    ComplainData,
    VBW_SEEDS_RESOURE_MAP,
    VBW_SEEDS_TEXTURE_DATA,
    VBW_SEEDS_COMPLAIN_DATA,
    VBW_SEEDS_TEXTURE_COUNT,
    ResoureStatus,
    ErrorCode,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///Add new IPFS texture
pub fn texture_add(
    ctx: Context<AddTexture>,      //default from system
    _index:u32,                     //texture id
    ipfs:String,                    //IPFS cid
) -> Result<()> {

    //1.check input
    //1.1. check ipfs format
    //1.2. check wether IPFS file is exsisted    
    //1.3. check wether index valid

    let clock = &ctx.accounts.clock;

    let payer_pubkey = ctx.accounts.payer.key();
    let owner=payer_pubkey.to_string();
    let create=clock.slot;
    let status=ResoureStatus::Created as u32;
    *ctx.accounts.texture_data=TextureData{
        ipfs,
        owner,
        create,
        status,
    };

    Ok(())
}

pub fn texture_approve(
    ctx: Context<ApproveTexture>, 
    _index:u32,                      //texture index in queue
) -> Result<()> {

    let texture= &mut ctx.accounts.texture_data;
    texture.status=ResoureStatus::Approved as u32;

    Ok(())
}

pub fn texture_complain(
    ctx: Context<ComplainTexture>, //default from system
    _index:u32,                      //texture index in queue
    complain:String,                     //complain JSON string
) -> Result<()> {
    
    let clock = &ctx.accounts.clock;
    let category=1;
    let result=String::from("{}");
    let create=clock.slot;
    *ctx.accounts.complain_data= ComplainData{
        category,
        complain,
        result,
        create,
    };

    Ok(())
}

pub fn texture_recover(
    ctx: Context<RecoverTexture>,      //default from system
    _index:u32,                    //texture index in queue
) -> Result<()> {

    let texture= &mut ctx.accounts.texture_data;
    texture.status=ResoureStatus::Approved as u32;
    
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
#[instruction(index:u32)]
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

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_COUNT],bump)]
    pub texture_counter: Account<'info, TextureCounter>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}   

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct ApproveTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub texture_data: Account<'info, TextureData>,
}


#[derive(Accounts)]
#[instruction(index:u32)]
pub struct ComplainTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub texture_data: Account<'info, TextureData>,

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
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(index:u32)]
pub struct RecoverTexture<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [VBW_SEEDS_TEXTURE_DATA,&index.to_le_bytes()],bump)]
    pub texture_data: Account<'info, TextureData>,
}