use base64::STANDARD;

use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action")]
pub(crate) enum Message {
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitPayload {
    shard_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecordPayload {
    #[serde(rename = "data", with = "Base64Standard")]
    raw_data: Vec<u8>,
    partition_key: String,
    sequence_number: String,
    sub_sequence_number: Option<u64>,
    approximate_arrival_timestamp: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProcessRecordPayload {
    records: Vec<RecordPayload>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckpointPayload {
    checkpoint: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckpointWithErrorPayload {
    checkpoint: String,
    error: String,
}

#[allow(dead_code)] // TODO: not used anywhere yet, remove when it's used
pub(crate) fn parse_message(payload: &str) -> JsonResult<Message> {
    // TODO: we should map to our own error types
    serde_json::from_str::<Message>(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initialize() {
        let given = "{\"action\" :\"initialize\", \"shardId\": \"shard1\"}";
        let expected = Message::Initialize(InitPayload {
            shard_id: "shard1".to_string(),
        });

        let actual = parse_message(given).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_process_record() {
        let given = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
        let parsed = parse_message(given).unwrap();

        if let Message::ProcessRecords(ProcessRecordPayload { records }) = parsed {
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
        } else {
            panic!("Did not match expected ProcessRecords event.");
        }
    }

    #[test]
    fn parse_checkpoint() {
        let given = "{\"action\": \"checkpoint\", \"checkpoint\": \"1234\", \"error\": \"check your point\"}";
        let expected = Message::Checkpoint(CheckpointWithErrorPayload {
            checkpoint: "1234".to_string(),
            error: "check your point".to_string(),
        });

        let actual = parse_message(given).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_lease_lost() {
        let given = "{\"action\": \"leaseLost\"}";
        let expected = Message::LeaseLost;

        let actual = parse_message(given).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_shard_end() {
        let given = "{\"action\": \"shardEnded\", \"checkpoint\": \"1234\"}";
        let expected = Message::ShardEnded(CheckpointPayload {
            checkpoint: "1234".to_string(),
        });

        let actual = parse_message(given).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_shutdown_requested() {
        let given = "{\"action\": \"shutdownRequested\", \"checkpoint\": \"1234\"}";
        let expected = Message::ShutdownRequested(CheckpointPayload {
            checkpoint: "1234".to_string(),
        });

        let actual = parse_message(given).unwrap();
        assert_eq!(actual, expected)
    }
}
