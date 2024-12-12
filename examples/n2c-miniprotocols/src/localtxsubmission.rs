// use std::collections::BTreeSet;

use pallas::{
    // codec::utils::Bytes,
    // ledger::{addresses::Address, traverse::MultiEraBlock},
    network::{
        facades::NodeClient,
        miniprotocols::{
            localtxsubmission::{self, EraTx, Message, RejectReason},
        },
    },
};
use tracing::info;

pub async fn do_local_tx_submission(client: &mut NodeClient) {
    let client = client.submission();

    let era_tx: EraTx = cbor_to_eratx(include_str!("../../../test_data/babbage4.tx"));

    let result = client.submit_tx(era_tx).await.unwrap();
    info!("result: {:02x?}", result);

    client.terminate_gracefully().await.unwrap();
}

fn cbor_to_eratx(input: &str) -> EraTx {
    EraTx(
        6,
        hex::decode(input).unwrap(),
    )
}
