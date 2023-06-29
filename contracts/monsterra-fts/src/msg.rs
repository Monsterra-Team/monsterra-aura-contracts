use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Timestamp, Uint128};
use cw20::Logo;
use cw20_base::msg::{ExecuteMsg as CW20ExecuteMsg, QueryMsg as CW20ueryMsg};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    /// Transfer is a base message to move tokens to another account without triggering actions
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    /// Burn is a base message to destroy tokens forever
    Burn {
        amount: Uint128,
    },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Only with "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Destroys tokens forever
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
    /// Only with the "mintable" extension. If authorized, creates amount new tokens
    /// and adds to the recipient balance.
    Mint {
        recipient: String,
        amount: Uint128,
    },
    /// Only with the "mintable" extension. The current minter may set
    /// a new minter. Setting the minter to None will remove the
    /// token's minter forever.
    UpdateMinter {
        new_minter: Option<String>,
    },
    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),

    TransferOwnerShip {
        user: Addr,
    },
    SetAdmin {
        user: Addr,
        status: bool,
    },
    SetSigner {
        public_key: Binary,
    },
    MintWithSignature {
        msg: MintMsg,
        signature: Binary,
    },
}

impl From<ExecuteMsg> for CW20ExecuteMsg {
    fn from(msg: ExecuteMsg) -> CW20ExecuteMsg {
        match msg {
            ExecuteMsg::Transfer { recipient, amount } => {
                CW20ExecuteMsg::Transfer { recipient, amount }
            }
            ExecuteMsg::Burn { amount } => CW20ExecuteMsg::Burn { amount },
            ExecuteMsg::Send {
                contract,
                amount,
                msg,
            } => CW20ExecuteMsg::Send {
                contract,
                amount,
                msg,
            },
            ExecuteMsg::IncreaseAllowance {
                spender,
                amount,
                expires,
            } => CW20ExecuteMsg::IncreaseAllowance {
                spender,
                amount,
                expires,
            },
            ExecuteMsg::DecreaseAllowance {
                spender,
                amount,
                expires,
            } => CW20ExecuteMsg::DecreaseAllowance {
                spender,
                amount,
                expires,
            },
            ExecuteMsg::TransferFrom {
                owner,
                recipient,
                amount,
            } => CW20ExecuteMsg::TransferFrom {
                owner,
                recipient,
                amount,
            },
            ExecuteMsg::SendFrom {
                owner,
                contract,
                amount,
                msg,
            } => CW20ExecuteMsg::SendFrom {
                owner,
                contract,
                amount,
                msg,
            },
            ExecuteMsg::BurnFrom { owner, amount } => CW20ExecuteMsg::BurnFrom { owner, amount },
            ExecuteMsg::Mint { recipient, amount } => CW20ExecuteMsg::Mint { recipient, amount },
            ExecuteMsg::UpdateMinter { new_minter } => CW20ExecuteMsg::UpdateMinter { new_minter },
            ExecuteMsg::UpdateMarketing {
                project,
                description,
                marketing,
            } => CW20ExecuteMsg::UpdateMarketing {
                project,
                description,
                marketing,
            },
            ExecuteMsg::UploadLogo(logo) => CW20ExecuteMsg::UploadLogo(logo),
            _ => panic!("cannot covert to CW20ExecuteMsg"),
        }
    }
}

#[cw_serde]
pub struct MintMsg {
    pub amount: Uint128,
    pub nonce: String,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct MintPayload {
    pub sender: Addr,
    pub amount: Uint128,
    pub nonce: String,
    pub timestamp: Timestamp,
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
    /// Returns the current balance of the given address, 0 if unset.
    #[returns(cw20::BalanceResponse)]
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(cw20::TokenInfoResponse)]
    TokenInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    #[returns(cw20::MinterResponse)]
    Minter {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(cw20::AllowanceResponse)]
    Allowance { owner: String, spender: String },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    #[returns(cw20::AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this spender has been granted. Supports pagination.
    #[returns(cw20::AllSpenderAllowancesResponse)]
    AllSpenderAllowances {
        spender: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    #[returns(cw20::AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    #[returns(cw20::MarketingInfoResponse)]
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    #[returns(cw20::DownloadLogoResponse)]
    DownloadLogo {},

    #[returns(Addr)]
    GetOwner {},
    #[returns(bool)]
    IsAdmin { user: Addr },
    #[returns(Binary)]
    GetSigner {},
    #[returns(bool)]
    IsUsedNonce { nonce: String },
}

impl From<QueryMsg> for CW20ueryMsg {
    fn from(msg: QueryMsg) -> CW20ueryMsg {
        match msg {
            QueryMsg::Balance { address } => CW20ueryMsg::Balance { address },
            QueryMsg::TokenInfo {} => CW20ueryMsg::TokenInfo {},
            QueryMsg::Minter {} => CW20ueryMsg::Minter {},
            QueryMsg::Allowance { owner, spender } => CW20ueryMsg::Allowance { owner, spender },
            QueryMsg::AllAllowances {
                owner,
                start_after,
                limit,
            } => CW20ueryMsg::AllAllowances {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllSpenderAllowances {
                spender,
                start_after,
                limit,
            } => CW20ueryMsg::AllSpenderAllowances {
                spender,
                start_after,
                limit,
            },
            QueryMsg::AllAccounts { start_after, limit } => {
                CW20ueryMsg::AllAccounts { start_after, limit }
            }
            QueryMsg::MarketingInfo {} => CW20ueryMsg::MarketingInfo {},
            QueryMsg::DownloadLogo {} => CW20ueryMsg::DownloadLogo {},
            _ => panic!("cannot covert to CW20QueryMsg"),
        }
    }
}
