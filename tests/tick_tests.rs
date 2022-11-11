mod mocks;

use amazon_kinesis_client::tick;

use crate::mocks::mock_processor::MockProcessor;
use crate::mocks::mock_reader::MockReader;

fn tick_into_processor(message: &str) -> MockProcessor {
    let mut reader = MockReader::with_input(message.to_string());
    let mut processor = MockProcessor::new();

    tick(&mut processor, &mut reader).unwrap();

    processor
}

#[test]
fn test_tick_initialize() {
    let message = "{\"action\" :\"initialize\", \"shardId\": \"shard1\"}";
    let processor = tick_into_processor(message);

    assert_eq!(processor.shard, Some("shard1".to_owned()));
}

#[test]
fn test_tick_new_record() {
    let message = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
    let processor = tick_into_processor(message);

    let record = processor.records.last().unwrap();
    assert_eq!(
        std::str::from_utf8(record.raw_data.as_slice()).unwrap(),
        "Hello, this is a test."
    )
}

#[test]
fn test_tick_lease_lost() {
    let message = "{\"action\": \"leaseLost\"}";
    let processor = tick_into_processor(message);

    assert!(processor.lease_lost);
}

#[test]
fn test_tick_shard_ended() {
    let message = "{\"action\": \"shardEnded\", \"checkpoint\": \"1234\"}";
    let processor = tick_into_processor(message);

    assert!(processor.shard_ended);
}

#[test]
fn test_tick_shutdown_requested() {
    let message = "{\"action\": \"shutdownRequested\", \"checkpoint\": \"1234\"}";
    let processor = tick_into_processor(message);

    assert!(processor.shutdown_requested);
}
