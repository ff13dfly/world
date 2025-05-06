use {
    //std::str::FromStr,
    anchor_lang::prelude::*,
    //anchor_lang::system_program,
};

// use crate::constants::{
//     SOLANA_PDA_LEN,
// };

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

///!important, init the VBW system.

pub fn init(
    _ctx: Context<InitVBW>,      //default from system
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
}

#[derive(Accounts)]
pub struct NewWorld<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}