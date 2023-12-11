use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("NotMinter")]
    NotMinter {},

    #[error("NotAdmin")]
    NotAdmin {},

    #[error("NotExistedNFT")]
    NotExistedNFT {},
    
    #[error("NotOwnedNFT")]
    NotOwnedNFT {},

    #[error("InvalidBoxContract")]
    InvalidBoxContract {},

    #[error("InvalidNftInfo")]
    InvalidNftInfo {},

    #[error("TimeExpired")]
    TimeExpired {},

    #[error("InvalidSignature")]
    InvalidSignature {},

    #[error("Internal")]
    Internal {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
