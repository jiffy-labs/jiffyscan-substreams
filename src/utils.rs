
use std::vec;

use crate::abi::entry_point::events::UserOperationEvent;
use crate::pb::gtms::user_operations::v1::{UserOperationInput, DecodedCallData};
use ethabi::ParamType;
use substreams_ethereum::pb::eth::v2::{BigInt, transaction_trace};
use substreams_ethereum::block_view::ReceiptView;
use crate::abi::entry_point::functions::HandleOps;
use substreams_ethereum::pb::eth::v2::TransactionTrace;
use substreams::{log, Hex};
use num_bigint::BigInt as num_BI;


pub fn get_user_op_input(sender: String, nonce: String, transaction: TransactionTrace) -> UserOperationInput {
    let mut handle_ops_input = HandleOps { ops: vec![], beneficiary: vec![] };
    let mut user_op_input: UserOperationInput = UserOperationInput { sender: String::new(), nonce: String::new(), init_code: String::new(), call_data: String::new(), call_gas_limit: String::new(), verification_gas_limit: String::new(), pre_verification_gas: String::new(), max_fee_per_gas: String::new(), max_priorit_fee_per_gas: String::new(), paymaster_and_data: String::new(), signature: String::new(), beneficiary: String::new(), decoded_call_data: None };
    let beneficiary;
    
    transaction.calls().for_each(|call| {
        log::info!("Trace call details call_type: {} \n, depth: {} \n, parent_index: {}", call.call.call_type, call.call.depth, call.call.parent_index);
        

        match HandleOps::decode(&call.call) {
            Ok(handle_ops) => {
                
                handle_ops_input = handle_ops
            }
            Err(_) => ()
        }
    });

    beneficiary = Hex::encode(handle_ops_input.beneficiary).to_string();

    handle_ops_input.ops.iter().for_each(|user_op| {
        
        if sender.eq_ignore_ascii_case(&Hex::encode(user_op.0.clone()).to_string()) && nonce.eq(&user_op.1.to_string()) {
            user_op_input = UserOperationInput {
                sender: vec_u8_to_byte_string(user_op.0.clone()),
                nonce: user_op.1.to_string(),
                init_code: vec_u8_to_byte_string(user_op.2.clone()),
                call_data: vec_u8_to_byte_string(user_op.3.clone()),
                call_gas_limit: user_op.4.to_string(),
                verification_gas_limit: user_op.5.to_string(),
                pre_verification_gas: user_op.6.to_string(),
                max_fee_per_gas: user_op.7.to_string(),
                max_priorit_fee_per_gas: user_op.8.to_string(),
                paymaster_and_data: vec_u8_to_byte_string(user_op.9.clone()),
                signature: vec_u8_to_byte_string(user_op.10.clone()),
                beneficiary: append_0x(beneficiary.clone()),
                decoded_call_data: Some(get_decoded_call_data(user_op.3.clone()))

            }
        }
    });

    user_op_input
}

fn get_decoded_call_data(call_data: Vec<u8>) -> DecodedCallData {
    // log::info!("call data {}", Hex::encode(call_data.clone()).to_string());

    let tokens = ethabi::decode(&[ParamType::Address, ParamType::Uint(256), ParamType::Bytes], &call_data[4..])
        .expect("failed to parse event payload");
    assert_eq!(tokens.len(), 3);

    // log::info!("target is {}", vec_u8_to_byte_string(tokens[0].clone().into_address().unwrap().as_bytes().to_vec()));
    
    DecodedCallData {
        targets: [vec_u8_to_byte_string(tokens[0].clone().into_address().unwrap().as_bytes().to_vec())].to_vec(),
        call_datas: [vec_u8_to_byte_string((tokens[2].clone().into_bytes().unwrap().to_vec()))].to_vec(),
        values: [append_0x(tokens[1].clone().into_uint().unwrap().to_string())].to_vec()
    }
}


pub fn string_to_bigint(str: String) -> substreams::scalar::BigInt {
    str.parse::<substreams::scalar::BigInt>()
        .expect("failed to parse str")
}

pub fn vec_u8_to_byte_string(vec: Vec<u8>) -> String {
    append_0x(Hex(vec).to_string())
}

fn append_0x(str: String) -> String {
    format!("{}{}", "0x", str)
}
