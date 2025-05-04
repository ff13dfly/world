use anchor_lang::prelude::*;


/********************************************************************/
/*********************** Static Parameters **************************/
/********************************************************************/

///Accounts space setting
pub const ANCHOR_DESCRIMINATOR_SIZE: usize = 8;
pub const SOLANA_PDA_LEN:usize=8;
pub const VBW_WHITELIST_MAP_SIZE:usize=1000;     //whitelist map size

///System setting
pub const VBW_ROOT_ACCOUNT:&str="GTNgXEzmG2E2d9yX8fwueP4bD2WCgJ3mqvt7sQj6CYYr"; //root of VBW program

///World setting
pub const VBW_WORLD_MAX:u32= 99;              //offset to get the block hash

///Block setting
pub const VBW_BLOCK_INIT_PRICE:u64= 1_000_000;      // 0.01 SOL, the block init price.

///PDA accounts seeds
pub const VBW_SEEDS_WHITE_LIST:&[u8;6]=b"manage";
pub const VBW_SEEDS_WORLD_LIST:&[u8;6]=b"worlds";
pub const VBW_SEEDS_WORLD_COUNT:&[u8;9]=b"w_counter";
pub const VBW_SEEDS_TEXTURE_BANNED_LIST:&[u8;10]=b"bd_texture";
pub const VBW_SEEDS_MODULE_BANNED_LIST:&[u8;9]=b"bd_module";
pub const VBW_SEEDS_BLOCK_BANNED_LIST:&[u8;8]=b"bd_block";
pub const VBW_SEEDS_WORLD_RECIPIENT:&[u8;11]=b"w_recipient";
pub const VBW_SEEDS_BLOCK_DATA:&[u8;6]=b"b_data";

#[account]
pub struct HoldingAccount {
    pub data: String,       //Sample Account to avoid error.
}

/********************************************************************/
/************************* World Related ****************************/
/********************************************************************/

//single VBW world setting
#[account]
pub struct WorldData {
    pub data: String,       //JSON world setting
    pub creator: String,    //creator of gene to accept token
    pub status: u32,        //wether close to mint
    pub start: u64,         //world start slot height
    pub released: u64,      //all blocks are sold out slot height
}

/********************************************************************/
/************************* Block Related ****************************/
/********************************************************************/

//single VBW block setting
#[account]
pub struct BlockData {
    pub data: String,     //JSON world setting
    pub owner: String,    //creator of gene to accept token
    pub create: u64,      //create slot height
    pub update: u64,      //update slot height
}

//the total supply of LUCK token
#[account]
#[derive(InitSpace)]
pub struct WorldCounter {
    pub value: u64,
}
impl WorldCounter {
    pub fn inc(&mut self, amount:u64) {
        self.value += amount
    }

    ///!important, only on Devnet
    //FIXME, DEBUG only, need to remove when deploy on mainnet
    pub fn set(&mut self, amount:u64) {
        self.value = amount
    }

    ///!important, only on Devnet
    //FIXME, DEBUG only, need to remove when deploy on mainnet
    pub fn max(&mut self) {
        self.value = 4096 * 4096
    }
}

/********************************************************************/
/*********************** Resource Related ***************************/
/********************************************************************/

//whitelist of managers, allow to manage the world
#[account]
pub struct WhiteList{
    pub data: Vec<String>,
    pub recipient:String,
}

impl WhiteList {
    pub fn push(&mut self, manager:String) {
        self.data.push(manager)
    }

    pub fn replace(&mut self, root:String) {
        self.data[0] = root
    }
    
    pub fn remove(&mut self,manager:String) {
        self.data.retain(|p| p != &manager);
    }

    pub fn recipient(&mut self,recipient:String){
        self.recipient = recipient
    }
}

//Texture queue for all worlds, texture can be banned
#[account]
pub struct TextureData{
    pub ipfs: String,   
    pub status: u32,
}

//Module queue for all worlds, texture can be banned
#[account]
pub struct ModuleData{
    pub ipfs: String,
    pub status: u32,
}