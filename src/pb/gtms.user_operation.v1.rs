// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperations {
    #[prost(message, repeated, tag="1")]
    pub user_operation: ::prost::alloc::vec::Vec<UserOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperation {
    #[prost(bytes="vec", tag="1")]
    pub user_op_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub paymaster: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub nonce: u64,
    #[prost(bool, tag="5")]
    pub success: bool,
    #[prost(string, tag="6")]
    pub actual_gas_cost: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub actual_gas_user: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
