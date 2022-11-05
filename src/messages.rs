extern crate base64_serde;
use base64::STANDARD;

use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
enum Message {
    #[serde(rename = "initialize")]
    Initialize(InitPayload),
    #[serde(rename = "processRecords")]
    ProcessRecords(ProcessRecordPayload),
    #[serde(rename = "checkpoint")]
    Checkpoint(CheckpointWithErrorPayload),
    #[serde(rename = "leaseLost")]
    LeaseLost,
    #[serde(rename = "shardEnded")]
    ShardEnded(CheckpointPayload),
    #[serde(rename = "shutdownRequested")]
    ShutdownRequested(CheckpointPayload),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitPayload {
    shard_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecordPayload {
    #[serde(rename = "data", with = "Base64Standard")]
    raw_data: Vec<u8>,
    partition_key: String,
    sequence_number: String,
    sub_sequence_number: Option<u64>,
    approximate_arrival_timestamp: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProcessRecordPayload {
    records: Vec<RecordPayload>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckpointPayload {
    checkpoint: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckpointWithErrorPayload {
    checkpoint: String,
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initialize() {
        let given = "{\"action\" :\"initialize\", \"shardId\": \"shard1\"}";
        let parsed = serde_json::from_str::<Message>(given);

        match parsed.unwrap() {
            Message::Initialize(InitPayload { shard_id }) => assert_eq!(shard_id, "shard1"),
            _ => panic!("Did not match expected Initialize event."),
        }
    }

    #[test]
    fn parse_process_record() {
        let given = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
        let parsed = serde_json::from_str::<Message>(given);

        match parsed.unwrap() {
            Message::ProcessRecords(ProcessRecordPayload { records }) => {
                assert_eq!(records.len(), 1);
                assert_eq!(
                    records[0].sequence_number,
                    "49590338271490256608559692538361571095921575989136588898"
                );
                assert_eq!(records[0].approximate_arrival_timestamp, 1570887011763.01);
                // TODO check if we can do this in serde
                assert_eq!(
                    "Hello, this is a test.",
                    String::from_utf8_lossy(&records[0].raw_data)
                );
            }
            _ => panic!("Did not match expected ProcessRecords event."),
        }
    }

    #[test]
    fn parse_checkpoint() {
        let given = "{\"action\": \"checkpoint\", \"checkpoint\": \"1234\", \"error\": \"check your point\"}";
        let parsed = serde_json::from_str::<Message>(given);

        match parsed.unwrap() {
            Message::Checkpoint(CheckpointWithErrorPayload { checkpoint, error }) => {
                assert_eq!(checkpoint, "1234");
                assert_eq!(error, "check your point");
            }
            _ => panic!("Did not match expected Checkpoint event."),
        }
    }

    #[test]
    fn parse_lease_lost() {
        let given = "{\"action\": \"leaseLost\"}";
        let parsed = serde_json::from_str::<Message>(given);
        match parsed.unwrap() {
            Message::LeaseLost => {}
            _ => panic!("Did not match expected LeaseLost event."),
        }
    }

    #[test]
    fn parse_shard_end() {
        let given = "{\"action\": \"shardEnded\", \"checkpoint\": \"1234\"}";
        let parsed = serde_json::from_str::<Message>(given);
        match parsed.unwrap() {
            Message::ShardEnded(CheckpointPayload { checkpoint }) => assert_eq!(checkpoint, "1234"),
            _ => panic!("Did not match expected ShardEnded event."),
        }
    }

    #[test]
    fn parse_shutdown_requested() {
        let given = "{\"action\": \"shutdownRequested\", \"checkpoint\": \"1234\"}";
        let parsed = serde_json::from_str::<Message>(given);
        match parsed.unwrap() {
            Message::ShutdownRequested(CheckpointPayload { checkpoint }) => {
                assert_eq!(checkpoint, "1234")
            }
            _ => panic!("Did not match expected ShardEnded event."),
        }
    }
}
