use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;
use cosmwasm_std::Timestamp;
use cw721_base::MinterResponse;
use schemars::JsonSchema;

use cosmwasm_std::Binary;
use cw721::Expiration;

use cw721_base::ExecuteMsg as CW721ExecuteMsg;
use cw721_base::InstantiateMsg as CW721InstantiateMsg;
use cw721_base::QueryMsg as CW721QueryMsg;

#[cw_serde]
pub struct MonsterraNFTInstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,

    /// Default URI of NFT contract
    pub base_uri: String,
}

impl From<MonsterraNFTInstantiateMsg> for CW721InstantiateMsg {
    fn from(msg: MonsterraNFTInstantiateMsg) -> CW721InstantiateMsg {
        match msg {
            MonsterraNFTInstantiateMsg {
                name,
                symbol,
                minter,
                base_uri: _,
            } => CW721InstantiateMsg {
                name,
                symbol,
                minter,
            },
        }
    }
}

#[cw_serde]
pub struct MonsterraNFTMigrateMsg {}

#[cw_serde]
pub enum MonsterraNFTExecuteMsg<T, E> {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },

    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        /// Unique ID of the NFT
        token_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: T,
    },
    /// Burn an NFT the sender has access to
    Burn {
        token_id: String,
    },

    /// Extension msg
    Extension {
        msg: E,
    },

    SetAdmin {
        user: Addr,
        status: bool,
    },

    SetSigner {
        public_key: Binary,
    },

    MintBatch(MintBatchMsg<T>),

    StakeBatch {
        token_ids: Vec<String>,
    },

    MintBatchWithSignature {
        msg: MintBatchWithSignatureMsg,
        signature: Binary,
    },

    SetBaseUri {
        base_uri: String,
    },
}

impl<T, E> From<MonsterraNFTExecuteMsg<T, E>> for CW721ExecuteMsg<T, E> {
    fn from(msg: MonsterraNFTExecuteMsg<T, E>) -> CW721ExecuteMsg<T, E> {
        match msg {
            MonsterraNFTExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => CW721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            MonsterraNFTExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => CW721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            MonsterraNFTExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => CW721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            MonsterraNFTExecuteMsg::Revoke { spender, token_id } => {
                CW721ExecuteMsg::Revoke { spender, token_id }
            }
            MonsterraNFTExecuteMsg::ApproveAll { operator, expires } => {
                CW721ExecuteMsg::ApproveAll { operator, expires }
            }
            MonsterraNFTExecuteMsg::RevokeAll { operator } => {
                CW721ExecuteMsg::RevokeAll { operator }
            }
            MonsterraNFTExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => CW721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            },
            _ => panic!("cannot covert to CW721ExecuteMsg"),
        }
    }
}

#[cw_serde]
pub struct MintMsg<T> {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,
}

#[cw_serde]
pub struct MintBatchMsg<T> {
    pub msgs: Vec<MintMsg<T>>,
}

#[cw_serde]
pub struct MintBatchWithSignatureMsg {
    pub token_ids: Vec<String>,
    pub nonce: String,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct MintBatchWithSignaturePayload {
    pub sender: Addr,
    pub token_ids: Vec<String>,
    pub nonce: String,
    pub timestamp: Timestamp,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MonsterraNFTQueryMsg<Q: JsonSchema> {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<Q>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<Q>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},

    /// Extension query
    #[returns(())]
    Extension { msg: Q },

    #[returns(bool)]
    IsUsedNonce { nonce: String },

    #[returns(bool)]
    IsAdmin { user: Addr },

    #[returns(Binary)]
    GetSigner {},

    #[returns(String)]
    GetBaseURI {},
}

impl<Q: JsonSchema> From<MonsterraNFTQueryMsg<Q>> for CW721QueryMsg<Q> {
    fn from(msg: MonsterraNFTQueryMsg<Q>) -> CW721QueryMsg<Q> {
        match msg {
            MonsterraNFTQueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => CW721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            MonsterraNFTQueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => CW721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            MonsterraNFTQueryMsg::Approvals {
                token_id,
                include_expired,
            } => CW721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            MonsterraNFTQueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => CW721QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            },
            MonsterraNFTQueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => CW721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            MonsterraNFTQueryMsg::NumTokens {} => CW721QueryMsg::NumTokens {},
            MonsterraNFTQueryMsg::ContractInfo {} => CW721QueryMsg::ContractInfo {},
            MonsterraNFTQueryMsg::NftInfo { token_id } => CW721QueryMsg::NftInfo { token_id },
            MonsterraNFTQueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => CW721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            MonsterraNFTQueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => CW721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            MonsterraNFTQueryMsg::AllTokens { start_after, limit } => {
                CW721QueryMsg::AllTokens { start_after, limit }
            }
            MonsterraNFTQueryMsg::Minter {} => CW721QueryMsg::Minter {},
            _ => panic!("cannot covert to CW721QueryMsg"),
        }
    }
}
