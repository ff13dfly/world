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

pub fn update(
    _ctx: Context<UpdateAdjunct>,      //default from system
    _short:String,
    _name:String,
    _format:String,
    _version:u32,
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
pub struct UpdateAdjunct<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}