#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");

use {
    world::*,
    block::*,
    resource::*,
};
pub mod world;
pub mod block;
pub mod resource;
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
}
