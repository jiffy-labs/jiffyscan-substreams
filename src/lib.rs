mod abi;
mod pb;

use std::str::FromStr;

use hex_literal::hex;
use num_bigint::BigInt as num_BI;
use pb::gtms::user_operations::v1::UserOperation as UserOp;
use pb::gtms::user_operations::v1::UserOperations as UserOps;
use substreams::errors::Error;
use substreams::{log};
use substreams::Hex as hx;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams::scalar::BigInt as BI;

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
                    log_ordinal: _log.ordinal()
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn graph_out(user_operations: UserOps) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    
    for user_operation in user_operations.user_operations {
        log::info!("User operation event graphed out {}", substreams::Hex::encode(&user_operation.user_op_hash).to_string());
        tables
            .create_row("UserOperations", substreams::Hex::encode(&user_operation.user_op_hash).to_string())
            .set("user_op_hash", user_operation.user_op_hash)
            .set("sender", user_operation.sender)
            .set("paymaster", user_operation.paymaster)
            .set("nonce", user_operation.nonce)
            .set("success", user_operation.success)
            // .set("actual_gas_cost", user_operation.actual_gas_cost)
            .set("actual_gas_used", user_operation.actual_gas_used)
            .set("actual_gas_cost", &user_operation.actual_gas_cost);
    }
    log::info!("Items in table values:{} keys:{}", tables.tables.values().len(), tables.tables.keys().len());
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(user_operations: UserOps) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for user_operation in user_operations.user_operations {
        let id = substreams::Hex::encode(&user_operation.user_op_hash).to_string();
        log::info!("User operation event db out {}", &id);

        db_out.push_change("UserOperations", &id, user_operation.log_ordinal, table_change::Operation::Update)
            .change("user_op_hash", ("", id.as_str()))
            .change("sender", (None, hx::encode(user_operation.sender).as_str()))
            .change("paymaster", (None, hx::encode(user_operation.paymaster).as_str()))
            .change("nonce", (None, str_to_big_int(user_operation.nonce)))
            .change("success", (false, user_operation.success))
            // .set("actual_gas_cost", user_operation.actual_gas_cost)
            .change("actual_gas_used", (None, str_to_big_int(user_operation.actual_gas_used)))
            .change("actual_gas_cost", (None, str_to_big_int(user_operation.actual_gas_cost)));
    }

    Ok(db_out)
}

fn str_to_big_int(str: String) -> BI {
    match BI::from_str(&str) {
        Ok(val) => val,
        Err(_) => BI::from(0)
    }
}

// #[substreams::handlers::store]
// fn store_tokens(user_operations: UserOps, store: store::StoreSet) {
//     for user_operation in user_operations.user_operations {
//         let key = format!("user_operation:{}", user_operation.user_op_hash);
//         store.set(user_operation.log_ordinal, key, &proto::encode(&user_operation).unwrap());
//     }
// }
