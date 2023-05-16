use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

pub struct GamePaymentContract<'a>
{
    pub contract_info: Item<'a, ContractInfo>,
    pub owner: Item<'a,Addr>,
    pub contract_supports: IndexedMap<'a, &'a Addr, ContractSupport, ContractSupportedIndexes<'a>>,
    pub token_payments: IndexedMap<'a, &'a str, PaymentMethod, PaymentMethodIndexes<'a>>,
}

impl<> Default for GamePaymentContract<'static>
{
    fn default() -> Self {
        Self::new(
            "contract_info",
            "owner",
            "contract_supports",
            "contract",
            "token_payments",
            "method"
        )
    }
}

impl<'a> GamePaymentContract<'a>
{
    fn new(
        contract_info: &'a str,
        owner: &'a str,
        contract_support_keys: &'a str,
        contract: &'a str,
        payments_key: &'a str,
        key: &'a str,
    ) -> Self {
        let indexes_contract = ContractSupportedIndexes {
            contract: MultiIndex::new(contract_support_idx, contract_support_keys, contract),
        };
        let indexes_payment = PaymentMethodIndexes {
            method: MultiIndex::new(payment_method_idx, payments_key, key),
        };
        Self {
            contract_info: Item::new(contract_info),
            owner: Item::new(owner),
            contract_supports: IndexedMap::new(contract_support_keys, indexes_contract),
            token_payments: IndexedMap::new(payments_key, indexes_payment),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
    pub total_contract_supported: u32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractSupport {
    pub contract_address: Addr,
    pub fee: u16,
    pub is_cw721: bool,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PaymentMethod {
    pub contract_address: Addr, 
    pub payment_contract: Addr,
    pub status: bool,
}

pub struct ContractSupportedIndexes<'a>
{
    // pk goes to second tuple element
    pub contract: MultiIndex<'a, Addr, ContractSupport, String>,
}

impl<'a> IndexList<ContractSupport> for ContractSupportedIndexes<'a>
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<ContractSupport>> + '_> {
        let v: Vec<&dyn Index<ContractSupport>> = vec![&self.contract];
        Box::new(v.into_iter())
    }
}

pub struct PaymentMethodIndexes<'a>
{
    // pk goes to second tuple element
    pub method: MultiIndex<'a, String, PaymentMethod, String>,
}

impl<'a> IndexList<PaymentMethod> for PaymentMethodIndexes<'a>
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<PaymentMethod>> + '_> {
        let v: Vec<&dyn Index<PaymentMethod>> = vec![&self.method];
        Box::new(v.into_iter())
    }
}

pub fn contract_support_idx<>(d: &ContractSupport) -> Addr {
    d.contract_address.clone()
}

pub fn payment_method_idx<>(d: &PaymentMethod)-> String {
    d.contract_address.clone().to_string() + (&d.payment_contract.clone().to_string())
}