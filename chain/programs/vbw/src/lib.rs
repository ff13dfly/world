#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");

use {
    world::*,
    block::*,
    resource::*,
    manage::*,
};
pub mod world;
pub mod block;
pub mod resource;
pub mod manage;
pub mod constants;

#[program]
pub mod vbw {
    use super::*;

    ///init whole VBW system
    pub fn init(
        ctx: Context<InitVBW>,
    ) -> Result<()> {
        world::init(ctx)
    }

    pub fn start_world(
        ctx: Context<NewWorld>,
        index:u32,
        setting:String,
    ) -> Result<()> {
        world::start(ctx,index,setting)
    }

    ///Block part
    pub fn mint_block(
        ctx: Context<MintBlock>,
        x: u32,
        y:u32,
        world:u32
    ) -> Result<()> {
        block::mint(ctx,x,y,world)
    }

    pub fn update_block(
        ctx: Context<UpdateBlock>,
        account:String,
        x: u32,
        y:u32,
        world:u32
    ) -> Result<()> {
        block::update(ctx,account,x,y,world)
    }

    pub fn sell_block(
        ctx: Context<SellBlock>,
        x: u32,
        y:u32,
        world:u32,
        price:u32,
    ) -> Result<()> {
        block::sell(ctx,price,x,y,world)
    }

    pub fn buy_block(
        ctx: Context<BuyBlock>,
        x: u32,
        y:u32,
        world:u32,
    ) -> Result<()> {
        block::buy(ctx,x,y,world)
    }

    pub fn complain_block(
        ctx: Context<ComplainBlock>,
        data:String,
        x: u32,
        y:u32,
        world:u32,
    ) -> Result<()> {
        block::complain(ctx,data,x,y,world)
    }

    pub fn recover_block(
        ctx: Context<RecoverBlock>,
        x: u32,
        y:u32,
        world:u32,
    ) -> Result<()> {
        block::recover(ctx,x,y,world)
    }

    pub fn ban_block(
        ctx: Context<BanBlock>,
        x: u32,
        y:u32,
        world:u32,
    ) -> Result<()> {
        manage::block(ctx,x,y,world)
    }

    ///Texture management
    pub fn add_texture(
        ctx: Context<AddTexture>,
        ipfs: String,
    ) -> Result<()> {
        texture::add(ctx,ipfs)
    }

    pub fn approve_texture(
        ctx: Context<ApproveTexture>,
        index: u64,
    ) -> Result<()> {
        texture::approve(ctx,index)
    }

    pub fn complain_texture(
        ctx: Context<ComplainTexture>,
        index: u64,
    ) -> Result<()> {
        texture::complain(ctx,index)
    }

    pub fn recover_texture(
        ctx: Context<RecoverTexture>,
        index: u64,
    ) -> Result<()> {
        texture::recover(ctx,index)
    }

    pub fn ban_texture(
        ctx: Context<BanTexture>,
        index: u64,
    ) -> Result<()> {
        manage::texture(ctx,index)
    }

    ///Module management
    pub fn add_module(
        ctx: Context<AddModule>,
        ipfs: String,
    ) -> Result<()> {
        module::add(ctx,ipfs)
    }
}
