mod abi;
mod pb;

use hex_literal::hex;
use pb::gtms::user_operations;
use pb::gtms::user_operations::v1::UserOperation as UserOp;
use pb::gtms::user_operations::v1::UserOperations as UserOps;
use substreams::errors::Error;
use substreams::{log, proto, store};
use substreams_ethereum::pb::eth::v2 as eth;

const ENTRY_POINT_CONTRACT: [u8; 20] = hex!("5FF137D4b0FDCD49DcA30c7CF57E578a026d2789");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_user_operations(block: eth::Block) -> Result<UserOps, Error> {
    Ok(UserOps {
        user_operations: block
            .events::<abi::entry_point::events::UserOperationEvent>(&[&ENTRY_POINT_CONTRACT])
            .map(|(user_operation_event, _log)| {
                log::info!("User Operation Event detected");

                UserOp {
                    user_op_hash: user_operation_event.user_op_hash.to_vec(),
                    sender: user_operation_event.sender,
                    paymaster: user_operation_event.paymaster,
                    nonce: user_operation_event.nonce.to_string(),
                    success: user_operation_event.success,
                    actual_gas_cost: user_operation_event.actual_gas_cost.to_string(),
                    actual_gas_used: user_operation_event.actual_gas_used.to_string(),
                }
            })
            .collect(),
    })
}

// #[substreams::handlers::store]
// fn store_tokens(user_operations: UserOps, store: store::StoreSet) {
//     for user_operation in user_operations.user_operations {
//         let key = format!("user_operation:{}", user_operation.user_op_hash);
//         store.set(1, key, &proto::encode(&user_operation).unwrap());
//     }
// }
