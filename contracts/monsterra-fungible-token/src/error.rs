use cosmwasm_std::StdError;
use cw20_base::ContractError as CW20Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CW20(#[from] CW20Error),

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
