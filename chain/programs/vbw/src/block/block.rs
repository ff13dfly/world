use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

use crate::constants::{
    SOLANA_PDA_LEN,
    WorldCounter,
    BlockData,
    ComplainData,
    VBW_SEEDS_WORLD_COUNT,
    VBW_SEEDS_BLOCK_DATA,
    VBW_SEEDS_COMPLAIN_DATA,
    BlockStatus,
    ErrorCode,
};


/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

pub fn mint(
    ctx: Context<MintBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check
    //1.1 wether x overflow
    //1.2 wether y overflow
    //1.3 wether world is on sell

    //2. logical check

    //3. init block

    //let block = &mut ctx.accounts.block_data;
    let clock = &ctx.accounts.clock;

    let data=String::from("[]");
    let payer_pubkey = ctx.accounts.payer.key();
    let owner=payer_pubkey.to_string();
    let price:u32=0;
    let create=clock.slot.clone();
    let update=clock.slot;
    let status=BlockStatus::Public as u32;
    *ctx.accounts.block_data= BlockData{
        data,
        owner,
        price,
        create,
        update,
        status
    };

    //inc minted amount
    let minted = &mut ctx.accounts.world_counter;
    minted.inc();

    Ok(())
}

pub fn update(
    ctx: Context<UpdateBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    data:String,                 //block data storaged on chain
)-> Result<()> {

    //1. input check
    //1.1 wether x overflow
    //1.2 wether y overflow
    //1.3 wether world is on sell
    //1.4 wether vallid account address

    //2. update the account address on block
    let clock = &ctx.accounts.clock;
    let bk= &mut ctx.accounts.block_data;
    bk.data=data;
    bk.update=clock.slot;
    Ok(())
}


pub fn sell(
    ctx: Context<SellBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    price:u32,                      //Selling price in SOL
) -> Result<()> {

    //1. input check

    let clock = &ctx.accounts.clock;
    let bk= &mut ctx.accounts.block_data;
    bk.price=price;
    bk.update=clock.slot;
    bk.status=BlockStatus::Selling as u32;            //FIXME, here to set an enum to select

    Ok(())
}

pub fn buy(
    ctx: Context<BuyBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

    let payer_pubkey = ctx.accounts.payer.key();
    let owner=payer_pubkey.to_string();

    let clock = &ctx.accounts.clock;

    let bk= &mut ctx.accounts.block_data;
    bk.owner=owner;
    bk.update=clock.slot;
    bk.price=0;

    Ok(())
}


pub fn revoke(
    ctx: Context<RevokeBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

    // let payer_pubkey = ctx.accounts.payer.key();
    // let owner=payer_pubkey.to_string();

    let clock = &ctx.accounts.clock;

    let bk= &mut ctx.accounts.block_data;
    bk.update=clock.slot;
    bk.price=0;

    Ok(())
}

pub fn complain(
    ctx: Context<ComplainBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    complain:String,                     //complain JSON string
) -> Result<()> {

    //1. input check
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

pub fn recover(
    ctx: Context<RecoverBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

    let clock = &ctx.accounts.clock;
    let bk= &mut ctx.accounts.block_data;
    bk.update=clock.slot;
    bk.status=BlockStatus::Public as u32;;
    

    Ok(())
}


/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

// fn is_valid_name() -> bool{
//     return true;
// }

///!important, do not add payer.publickey as one of seeds. Need to sell/buy.
///!important, added "owner" in data struct, check that to confirm ownership.

/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/


#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct MintBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Record of block
    #[account(
        init_if_needed,
        space = SOLANA_PDA_LEN + BlockData::INIT_SPACE,
        payer = payer,
        seeds = [
            VBW_SEEDS_BLOCK_DATA,
            &x.to_le_bytes(),
            &y.to_le_bytes(),
            &world.to_le_bytes(),
        ],
        bump,
    )]
    pub block_data: Account<'info, BlockData >,

    #[account(
        init_if_needed,
        space = SOLANA_PDA_LEN + WorldCounter::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_WORLD_COUNT,
            &world.to_le_bytes(),
        ],
        bump,
    )]
    pub world_counter: Account<'info, WorldCounter>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct UpdateBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct SellBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct BuyBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct RevokeBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct ComplainBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    #[account(
        init,
        space = SOLANA_PDA_LEN + ComplainData::INIT_SPACE,     
        payer = payer,
        seeds = [
            VBW_SEEDS_COMPLAIN_DATA,    //need to set [u8;4] to avoid error
            &x.to_le_bytes(),
            &y.to_le_bytes(),
            &world.to_le_bytes(),
        ],
        bump,
    )]
    pub complain_data: Account<'info, ComplainData>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct RecoverBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,seeds = [
        VBW_SEEDS_BLOCK_DATA,
        &x.to_le_bytes(),
        &y.to_le_bytes(),
        &world.to_le_bytes(),
    ],bump)]
    pub block_data: Account<'info, BlockData>,

    pub clock: Sysvar<'info, Clock>,
}