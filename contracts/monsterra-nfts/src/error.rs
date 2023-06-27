use cosmwasm_std::StdError;
use cw721_base::ContractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonsterraNFTError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CW721(#[from] ContractError),

    #[error("Internal")]
    Internal {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("NonceUsed")]
    NonceUsed {},

    #[error("TimeExpired")]
    TimeExpired {},

    #[error("InvalidSignature")]
    InvalidSignature {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
