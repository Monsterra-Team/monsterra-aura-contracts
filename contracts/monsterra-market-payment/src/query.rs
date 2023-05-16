use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult, Addr};

use crate::msg::{QueryMsg};
use crate::state::{ContractInfo, GamePaymentContract, ContractSupport};

impl<'a> GamePaymentQuery<> for GamePaymentContract<'a>
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfo> {
        self.contract_info.load(deps.storage)
    }

    fn contract_support_info(&self, deps: Deps, contract_address: Addr) -> StdResult<ContractSupport> {
        let info = self.contract_supports.load(deps.storage, &contract_address)?;
        Ok(info)
    }

    fn is_token_support(&self, deps: Deps, contract_address: Addr, payment_contract: Addr) -> StdResult<bool> {
        let key = contract_address.to_string() + &payment_contract.to_string();
        let info = self.token_payments.load(deps.storage, &key);
        let result = match info {
            Ok(info) => info.status,
            Err(_) => false,
        };
        Ok(result)
    }

    fn get_contract_fee(&self, deps: Deps, contract_address: Addr) -> StdResult<u16> {
        let info = self.contract_supports.load(deps.storage, &contract_address);
        let result = match info {
            Ok(info) => info.fee,
            Err(_) => 0,
        };
        Ok(result)
    }
}

pub trait GamePaymentQuery<>
{
    fn contract_support_info(
        &self,
        deps: Deps,
        contract_address: Addr,
    ) -> StdResult<ContractSupport>;

    fn is_token_support(
        &self,
        deps: Deps,
        contract_address: Addr,
        payment_contract: Addr
    ) -> StdResult<bool>;

    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfo>;

    fn get_contract_fee(
        &self,
        deps: Deps,
        contract_address: Addr,
    ) -> StdResult<u16>;
}

impl<'a> GamePaymentContract<'a>
{
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GamePaymentContractInfo {} => to_binary(&self.contract_info(deps)?),
            
            QueryMsg::ContractSupportInfo {
                contract_address,
            } => to_binary(&self.contract_support_info(
                deps,
                contract_address,
            )?),
            QueryMsg::IsTokenSupport {
                contract_address,
                payment_contract
            } => to_binary(&self.is_token_support(
                deps,
                contract_address,
                payment_contract
            )?),
            QueryMsg::GetContractFee {
                contract_address,
            } => to_binary(&self.get_contract_fee(
                deps,
                contract_address,
            )?),
        }
    }
}