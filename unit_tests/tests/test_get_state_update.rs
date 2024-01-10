#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, BlockTag, StarknetError};
use starknet_providers::{
    jsonrpc::HttpTransport, JsonRpcClient, MaybeUnknownErrorCode, Provider, ProviderError,
    StarknetErrorWithMessage,
};
use std::assert_matches::assert_matches;
use std::collections::HashMap;

/// Test for the `get_state_update` Deoxys RPC method
/// # Arguments
// * `block_id` - The block id to get the state update from
//
// # Returns
// * `state update`
// or
// * `pending state update`
//
// # Errors
// * `block_not_found` - If the block is not found or invalid

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    assert_matches!(
        deoxys
        .get_state_update(BlockId::Number(0))
        .await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_state_update(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Deoxys : Error while getting the state update");
    let response_pathfinder = pathfinder
        .get_state_update(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("RPC : Error while getting the state update");

    assert_eq!(response_deoxys, response_pathfinder);
}