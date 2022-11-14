use base64::STANDARD;
use base64_serde::base64_serde_type;
use eyre::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

#[cfg(feature = "dynamodb-events")]
use crate::kinesis_dynamodb::DynamoDBPayload;

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

    #[cfg(feature = "dynamodb-events")]
    pub fn dynamodb_event(&self) -> JsonResult<DynamoDBPayload> {
        serde_json::from_slice::<DynamoDBPayload>(self.raw_data.as_slice())
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckpointWithErrorPayload {
    checkpoint: String,
    error: String,
}

pub(crate) fn parse_message(payload: &str) -> Result<Message> {
    let message = serde_json::from_str::<Message>(payload)?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "dynamodb-events")]
    use aws_lambda_events::dynamodb::attributes::AttributeValue;
    #[cfg(feature = "dynamodb-events")]
    use std::collections::HashMap;

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

    #[cfg(feature = "dynamodb-events")]
    #[test]
    fn parse_dynamodb_process_record() {
        let given = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"eyJhd3NSZWdpb24iOiJldS13ZXN0LTEiLCJldmVudElEIjoiOGFmMzhlOGUtZTIxMy00YjUyLWEwZWEtNzcxNDhiMGEwNzZkIiwiZXZlbnROYW1lIjoiSU5TRVJUIiwidXNlcklkZW50aXR5IjpudWxsLCJyZWNvcmRGb3JtYXQiOiJhcHBsaWNhdGlvbi9qc29uIiwidGFibGVOYW1lIjoidGVzdF90YWJsZSIsImR5bmFtb2RiIjp7IkFwcHJveGltYXRlQ3JlYXRpb25EYXRlVGltZSI6MTY2ODQxNjg2NjUyMCwiS2V5cyI6eyJuIjp7IlMiOiJwYXJ0aXRpb25fa2V5In0sInQiOnsiUyI6InNvcnRfa2V5In19LCJOZXdJbWFnZSI6eyJhIjp7Ik0iOnsieCI6eyJOIjoiMTQwLjE3In0sImMiOnsiTiI6IjE1In19fSwidHRsIjp7Ik4iOiIxNjY4ODQ4ODY2In0sInQiOnsiUyI6InNvcnRfa2V5In0sIm4iOnsiUyI6InBhcnRpdGlvbl9rZXkifX0sIlNpemVCeXRlcyI6MjA2fSwiZXZlbnRTb3VyY2UiOiJhd3M6ZHluYW1vZGIifQ==\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
        let parsed = parse_message(given).unwrap();

        if let Message::ProcessRecords(ProcessRecordPayload { records }) = parsed {
            assert_eq!(records.len(), 1);
            let actual = records[0].dynamodb_event().unwrap();
            let expected = DynamoDBPayload {
                dynamodb: aws_lambda_events::dynamodb::StreamRecord {
                    // TODO this timestamp is currently parsed in seconds instead of ms
                    // So it looks like "+54840-01-10T09:35:20Z"
                    approximate_creation_date_time: Default::default(),
                    keys: HashMap::from([
                        (
                            "n".to_string(),
                            AttributeValue::String("partition_key".to_string()),
                        ),
                        (
                            "t".to_string(),
                            AttributeValue::String("sort_key".to_string()),
                        ),
                    ]),
                    new_image: HashMap::from([
                        (
                            "a".to_string(),
                            AttributeValue::AttributeMap(HashMap::from([
                                ("x".to_string(), AttributeValue::Number(140.17)),
                                ("c".to_string(), AttributeValue::Number(15.0)),
                            ])),
                        ),
                        ("ttl".to_string(), AttributeValue::Number(1668848866.0)),
                        (
                            "t".to_string(),
                            AttributeValue::String("sort_key".to_string()),
                        ),
                        (
                            "n".to_string(),
                            AttributeValue::String("partition_key".to_string()),
                        ),
                    ]),
                    old_image: Default::default(),
                    sequence_number: None,
                    size_bytes: 206,
                    stream_view_type: None,
                },
                aws_region: "eu-west-1".to_string(),
                event_id: "8af38e8e-e213-4b52-a0ea-77148b0a076d".to_string(),
                event_name: "INSERT".to_string(),
                user_identity: None,
                record_format: "application/json".to_string(),
                table_name: "test_table".to_string(),
                event_source: "aws:dynamodb".to_string(),
            };
            assert_eq!(expected, actual);
        } else {
            panic!("Did not match expected DynamoDB event.");
        }
    }
}
