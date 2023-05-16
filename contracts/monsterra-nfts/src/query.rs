use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::msg::TransferLogResponse;

use crate::state::{TransferLog, TRANSFER_LOGS};

const MAX_LIMIT: u32 = 100;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_transfer_log(
  deps: Deps,
  start_after: Option<u64>,
  limit: Option<u32>,
) -> StdResult<TransferLogResponse> {
  let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
  let start = start_after.map(Bound::exclusive::<u64>);
  let logss = TRANSFER_LOGS
    .range(deps.storage, start, None, Order::Ascending)
    .take(limit)
    .map(|item| {
      item.map(|(_id, log)| TransferLog {
        from: log.from,
        to: log.to,
        token_id: log.token_id,
      })
    });

  let logs = logss.collect::<StdResult<_>>()?;
  Ok(TransferLogResponse { logs })
}
