create table user_operations
(
    id text not null constraint block_meta_pk primary key,
    user_op_hash text,
    sender text,
    paymaster text,
    nonce bigint,
    success boolean,
    actual_gas_used bigint,
    actual_gas_cost bigint,
    timestamp   timestamp
);
