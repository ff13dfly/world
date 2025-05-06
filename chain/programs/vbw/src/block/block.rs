use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

// use crate::constants::{
//     SOLANA_PDA_LEN,
//     BlockData,
//     VBW_SEEDS_BLOCK_DATA,
// };

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

pub fn mint(
    _ctx: Context<MintBlock>,      //default from system
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

    //let data = &mut ctx.accounts.block_data;

    Ok(())
}

pub fn update(
    _ctx: Context<UpdateBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    _account:String,                 //Account storage the block data
)-> Result<()> {

    //1. input check
    //1.1 wether x overflow
    //1.2 wether y overflow
    //1.3 wether world is on sell
    //1.4 wether vallid account address

    //2. update the account address on block

    Ok(())
}


pub fn sell(
    _ctx: Context<SellBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    _price:u32,                      //Selling price in SOL
) -> Result<()> {

    //1. input check

    Ok(())
}

pub fn buy(
    _ctx: Context<BuyBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

    Ok(())
}


pub fn revoke(
    _ctx: Context<RevokeBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

    Ok(())
}

pub fn complain(
    _ctx: Context<ComplainBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
    _data:String,                     //complain JSON string
) -> Result<()> {

    //1. input check

    Ok(())
}

pub fn recover(
    _ctx: Context<RecoverBlock>,      //default from system
    _x:u32,                      
    _y:u32,
    _world:u32,
) -> Result<()> {

    //1. input check

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
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct UpdateBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct SellBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct BuyBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct RevokeBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct ComplainBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(x:u32,y:u32,world:u32)]
pub struct RecoverBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}