use crate::AppchainId;

/// Storage keys for collections of sub-struct in main contract
pub enum StorageKey {
    AppchainIds,
    AppchainBasedatas,
    UpvoteDeposits,
    DownvoteDeposits,
    RegistrySettings,
    RegistryRoles,
    RegistryContractWasm,
    AppchainMetadata(AppchainId),
    AppchainAnchorCode(AppchainId),
    AppchainVotingScore(AppchainId),
}

impl StorageKey {
    pub fn to_string(&self) -> String {
        match self {
            StorageKey::AppchainIds => "i".to_string(),
            StorageKey::AppchainBasedatas => "a".to_string(),
            StorageKey::UpvoteDeposits => "u".to_string(),
            StorageKey::DownvoteDeposits => "d".to_string(),
            StorageKey::RegistrySettings => "rs".to_string(),
            StorageKey::RegistryRoles => "rr".to_string(),
            StorageKey::RegistryContractWasm => "rcw".to_string(),
            StorageKey::AppchainMetadata(appchain_id) => format!("{}md", appchain_id),
            StorageKey::AppchainAnchorCode(appchain_id) => format!("{}ac", appchain_id),
            StorageKey::AppchainVotingScore(appchain_id) => format!("{}vs", appchain_id),
        }
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
