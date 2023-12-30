pub struct ContractState {
    pub owner: String,
    pub minted_nfts: Vec<NFT>,
}

pub struct NFT {
    pub id: String,
    pub metadata_url: String,
    pub owner: String,
}

pub struct Ownership {
    pub owner: String,
    pub nfts: Vec<NFT>,
}

pub struct EventData {
    pub event_id: String,
    pub nft: NFT,
}

pub struct CrossChainData {
    pub chain: String,
    pub data: Vec<u8>,
}
