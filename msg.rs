pub struct InitMsg {
    pub owner: String,
    pub api_config: String,
    pub initial_settings: String,
}

pub struct MintMsg {
    pub event_id: String,
    pub recipient: String,
    pub metadata: String,
    pub minting_rule: MintingRule,
}

pub enum MintingRule {
    ByMinter { minter: String },
    ByKey { pubkey: String, signature: String },
    ByKeys { pubkey: Vec<String>, signature: String },
}

pub struct BurnMsg {
    pub nft_id: String,
    pub owner: String,
}

pub enum QueryMsg {
    NftDetails { nft_id: String },
    Ownership { owner: String },
    CrossChainData { chain: String },
}

pub enum ResponseMsg {
    Success { message: String },
    Error { message: String },
}
