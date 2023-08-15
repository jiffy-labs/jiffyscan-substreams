// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperations {
    #[prost(message, repeated, tag="1")]
    pub user_operations: ::prost::alloc::vec::Vec<UserOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperation {
    #[prost(string, tag="1")]
    pub user_op_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub paymaster: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub success: bool,
    #[prost(string, tag="6")]
    pub actual_gas_cost: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub actual_gas_used: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub block_info: ::core::option::Option<BlockInfo>,
    #[prost(message, optional, tag="10")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    #[prost(message, optional, tag="11")]
    pub user_op_input: ::core::option::Option<UserOperationInput>,
    #[prost(uint64, tag="12")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperationRevert {
    #[prost(bytes="vec", tag="1")]
    pub user_op_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub revert_reason: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    #[prost(int64, tag="1")]
    pub block_time: i64,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub transaction_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperationInput {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub init_code: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub call_data: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub call_gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub verification_gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub pre_verification_gas: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub max_fee_per_gas: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub max_priorit_fee_per_gas: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub paymaster_and_data: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(message, optional, tag="13")]
    pub decoded_call_data: ::core::option::Option<DecodedCallData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecodedCallData {
    #[prost(string, repeated, tag="1")]
    pub targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub call_datas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
