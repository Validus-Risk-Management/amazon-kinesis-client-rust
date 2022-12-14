use base64::STANDARD;

use base64_serde::base64_serde_type;
use eyre::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
use thiserror::Error;

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Debug, Deserialize, PartialEq)]
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
    pub shard_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "data", with = "Base64Standard")]
    pub raw_data: Vec<u8>,
    pub partition_key: String,
    pub sequence_number: String,
    pub sub_sequence_number: Option<u64>,
    pub approximate_arrival_timestamp: f64,
}

impl Record {
    pub fn json<T: DeserializeOwned>(&self) -> JsonResult<T> {
        serde_json::from_slice::<T>(self.raw_data.as_slice())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProcessRecordPayload {
    pub records: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckpointPayload {
    checkpoint: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckpointWithErrorPayload {
    pub(crate) checkpoint: Option<String>,
    pub(crate) error: Option<CheckpointError>,
}
// For more info, see https://github.com/awslabs/amazon-kinesis-client/tree/master/amazon-kinesis-client/src/main/java/software/amazon/kinesis/exceptions
#[derive(Debug, Deserialize, PartialEq, Eq, Error)]
#[serde(field_identifier)]
pub enum CheckpointError {
    // This is thrown when the Amazon Kinesis Client Library encounters issues talking to its dependencies
    // (e.g. fetching data from Kinesis, DynamoDB table reads/writes, emitting metrics to CloudWatch).
    #[error("KinesisClientLibDependencyException")]
    KinesisClientLibDependencyException,
    // Thrown when requests are throttled by a service (e.g. DynamoDB when storing a checkpoint).
    #[error("ThrottlingException")]
    ThrottlingException,
    // This is thrown when the Amazon Kinesis Client Library encounters issues with its internal state
    // (e.g. DynamoDB table is not found)
    #[error("InvalidStateException")]
    InvalidStateException,
    // The ShardRecordProcessor instance has been shutdown (e.g. and attempts a checkpoint).
    #[error("ShutdownException")]
    ShutdownException,
    // The MultiLang daemon sent us something that is not a checkpoint response, while waiting for one.
    #[serde(skip)]
    #[error("UnexpectedResponse")]
    UnexpectedResponse,
    // A catch-all exception for other errors, e,g, the MultiLang daemon sent us an error that is not defined.
    #[error("Exception: \"{0}\"")]
    Exception(String),
}

impl From<eyre::Report> for CheckpointError {
    fn from(e: eyre::Report) -> Self {
        CheckpointError::Exception(e.to_string())
    }
}

impl From<serde_json::Error> for CheckpointError {
    fn from(e: serde_json::Error) -> Self {
        CheckpointError::Exception(e.to_string())
    }
}

impl CheckpointError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CheckpointError::KinesisClientLibDependencyException
                | CheckpointError::ThrottlingException
        )
    }
}

pub(crate) fn parse_message(payload: &str) -> Result<Message> {
    let message = serde_json::from_str::<Message>(payload)?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::CheckpointError::InvalidStateException;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct DummyPayload {
        foo: String,
    }

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
    fn parse_json_process_record() {
        let given = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"eyJmb28iOiAiYmFyIn0=\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
        let parsed = parse_message(given).unwrap();

        if let Message::ProcessRecords(ProcessRecordPayload { records }) = parsed {
            assert_eq!(records.len(), 1);
            let actual = records[0].json::<DummyPayload>().unwrap();
            let expected = DummyPayload {
                foo: "bar".to_string(),
            };
            assert_eq!(expected, actual);
        } else {
            panic!("Did not match expected ProcessRecords event.");
        }
    }

    #[test]
    fn parse_checkpoint() {
        let given = "{\"action\": \"checkpoint\", \"checkpoint\": \"1234\", \"error\": \"InvalidStateException\"}";
        let expected = Message::Checkpoint(CheckpointWithErrorPayload {
            checkpoint: Some("1234".to_string()),
            error: Some(InvalidStateException),
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
