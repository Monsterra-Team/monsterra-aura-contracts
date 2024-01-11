use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{ContractInfo, ContractSupport, GamePaymentContract, PaymentMethod};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:game-payment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> GamePaymentContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract_info = ContractInfo {
            name: msg.name,
            symbol: msg.symbol,
            owner: _info.sender.clone(),
            total_contract_supported: 0,
        };
        self.contract_info.save(deps.storage, &contract_info)?;
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let contract_info = self.contract_info.load(deps.storage)?;

        if contract_info.owner != info.sender.clone() {
            return Err(ContractError::Unauthorized {});
        }
        match msg {
            ExecuteMsg::AddContractSupport {
                contract_address,
                fee,
                payment_contract,
                is_cw721,
            } => self.add_contract_support(
                deps,
                info,
                contract_address,
                fee,
                payment_contract,
                is_cw721,
            ),
            ExecuteMsg::UpdateFee {
                contract_address,
                fee,
            } => self.update_fee(deps, info, contract_address, fee),
            ExecuteMsg::SetPaymentMethod {
                contract_address,
                payment_contract,
                status,
            } => self.set_payment_method(deps, info, contract_address, payment_contract, status),
            ExecuteMsg::RemoveContractSupport { contract_address } => {
                self.remove_contract_support(deps, info, contract_address)
            }
            ExecuteMsg::TransferOwnerShip { user } => self.set_new_owner(deps, info, user),
        }
    }
}

pub trait GamePaymentExecute {
    type Err: ToString;

    fn add_contract_support(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        fee: u16,
        payment_contract: Addr,
        is_cw721: bool,
    ) -> Result<Response, Self::Err>;

    fn update_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        fee: u16,
    ) -> Result<Response, Self::Err>;

    fn set_payment_method(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        payment_contract: Addr,
        status: bool,
    ) -> Result<Response, Self::Err>;

    fn remove_contract_support(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
    ) -> Result<Response, Self::Err>;

    fn set_new_owner(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        user: Addr,
    ) -> Result<Response, ContractError>;
}

impl<'a> GamePaymentExecute for GamePaymentContract<'a> {
    type Err = ContractError;

    fn add_contract_support(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        fee: u16,
        payment_contract: Addr,
        is_cw721: bool,
    ) -> Result<Response, ContractError> {
        let contract = ContractSupport {
            contract_address: contract_address.clone(),
            fee,
            is_cw721,
            status: true,
        };
        let key = contract_address.clone().to_string() + &payment_contract.clone().to_string();
        let payment_method = PaymentMethod {
            contract_address: contract_address.clone(),
            status: true,
            payment_contract: payment_contract.clone(),
        };
        self.contract_supports
            .update(deps.storage, &contract_address, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(contract),
            })?;
        self.token_payments
            .update(deps.storage, &key, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(payment_method),
            })?;

        Ok(Response::new()
            .add_attribute("action", "add_contract_support")
            .add_attribute("sender", info.sender))
    }

    fn update_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        fee: u16,
    ) -> Result<Response, ContractError> {
        let mut contract = self
            .contract_supports
            .load(deps.storage, &contract_address)?;

        contract.fee = fee;
        self.contract_supports
            .save(deps.storage, &contract_address, &contract)?;
        // Send message
        Ok(Response::new()
            .add_attribute("action", "update_fee")
            .add_attribute("sender", info.sender))
    }

    fn set_payment_method(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
        payment_contract: Addr,
        status: bool,
    ) -> Result<Response, ContractError> {
        let key = contract_address.clone().to_string() + &payment_contract.clone().to_string();
        let payment_method = PaymentMethod {
            contract_address,
            status,
            payment_contract,
        };
        self.token_payments
            .update(deps.storage, &key, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(payment_method),
            })?;
        Ok(Response::new()
            .add_attribute("action", "set_payment_method")
            .add_attribute("sender", info.sender))
    }

    fn remove_contract_support(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        contract_address: Addr,
    ) -> Result<Response, ContractError> {
        let mut contract = self
            .contract_supports
            .load(deps.storage, &contract_address)?;
        contract.status = false;
        self.contract_supports
            .save(deps.storage, &contract_address, &contract)?;
        Ok(Response::new()
            .add_attribute("action", "remove_contract_support")
            .add_attribute("sender", info.sender))
    }

    fn set_new_owner(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        user: Addr,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        if contract_info.owner != info.sender.clone() {
            return Err(ContractError::Unauthorized {});
        }

        contract_info.owner = user.clone();

        self.contract_info.save(deps.storage, &contract_info)?;

        Ok(Response::new()
            .add_attribute("action", "tranfer_ownership")
            .add_attribute("owner", user.clone()))
    }
}

// helpers
impl<'a> GamePaymentContract<'a> {}
