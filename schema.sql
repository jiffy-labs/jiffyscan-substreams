create table user_operations
(
    id text not null constraint block_meta_pk primary key,
    user_op_hash text,
    sender text,
    paymaster text,
    nonce numeric(78,0),
    success boolean,
    actual_gas_used numeric(78,0),
    actual_gas_cost numeric(78,0),
    timestamp   timestamp
);

create table cursors
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);