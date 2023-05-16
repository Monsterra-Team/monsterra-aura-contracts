// use cosmwasm_std::StdError;
// use thiserror::Error;

// #[derive(Error, Debug, PartialEq)]
// pub enum ContractError {
//     #[error("{0}")]
//     Std(#[from] StdError),

//     #[error("Unauthorized")]
//     Unauthorized {},

//     #[error("Contract Address already added")]
//     Added {},

//     #[error("Cannot set approval that is already expired")]
//     Expired {},

//     #[error("Insufficien token balance")]
//     InsufficienTokenBalance {},

//     #[error("Invalid quantity")]
//     InvalidQuantity {},

//     #[error("Invalid number of item")]
//     InvalidNumberItem {},

//     #[error("Invalid price")]
//     InvalidPrice {},

//     #[error("Payment method not support")]
//     PaymentMethodNotSupport {},

//     #[error("Order canceled")]
//     OrderCanceled {},

//     #[error("Bid canceled")]
//     BidCanceled {},

//     #[error("Bundle canceled")]
//     BundleCanceled {},

//     #[error("Not owner")]
//     NotOwner {},

//     #[error("Only support Cw721")]
//     OnlySupportCw721 {},

//     #[error("Can not accept bid")]
//     CanNotAcceptBid {},

//     #[error("Bid expired")]
//     BidExpired {},
// }
