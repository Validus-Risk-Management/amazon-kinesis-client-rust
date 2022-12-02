mod mocks;

use kcl::tick;

use crate::mocks::mock_processor::MockProcessor;
use crate::mocks::mock_reader::MockReader;
use crate::mocks::mock_writer::MockWriter;

fn tick_into_processor(message: &str) -> (MockProcessor, MockWriter) {
    let mut processor = MockProcessor::default();
    let mut reader = MockReader::with_input(message.to_string());
    let mut writer = MockWriter::default();

    tick(&mut processor, &mut reader, &mut writer).unwrap();

    (processor, writer)
}

fn assert_status_response(writer: &MockWriter, status: &str) {
    let expected_out = format!("{{\"action\":\"status\",\"responseFor\":\"{status}\"}}\n");
    assert_eq!(writer.outputs.last(), Some(&expected_out))
}

#[test]
fn test_tick_initialize() {
    let message = "{\"action\" :\"initialize\", \"shardId\": \"shard1\"}";
    let (processor, writer) = tick_into_processor(message);

    assert_eq!(processor.shard, Some("shard1".to_owned()));
    assert_eq!(writer.outputs.len(), 1);
    assert_status_response(&writer, "initialize");
}

#[test]
fn test_tick_new_record() {
    let message = "{\"action\": \"processRecords\", \
        \"records\": [{\
            \"data\": \"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==\",\
            \"partitionKey\": \"1\",\
            \"sequenceNumber\": \"49590338271490256608559692538361571095921575989136588898\",\
            \"approximateArrivalTimestamp\": 1570887011763.01}]}";
    let (processor, writer) = tick_into_processor(message);

    let record = processor.records.last().unwrap();
    assert_eq!(
        std::str::from_utf8(record.raw_data.as_slice()).unwrap(),
        "Hello, this is a test."
    );
    assert_eq!(writer.outputs.len(), 1);
    assert_status_response(&writer, "processRecords");
}

#[test]
fn test_tick_lease_lost() {
    let message = "{\"action\": \"leaseLost\"}";
    let (processor, writer) = tick_into_processor(message);

    assert!(processor.lease_lost);
    assert_eq!(writer.outputs.len(), 1);
    assert_status_response(&writer, "leaseLost");
}

#[test]
fn test_tick_shard_ended() {
    let message = "{\"action\": \"shardEnded\", \"checkpoint\": \"1234\"}";
    let (processor, writer) = tick_into_processor(message);

    assert!(processor.shard_ended);
    assert_eq!(writer.outputs.len(), 1);
    assert_status_response(&writer, "shardEnded");
}

#[test]
fn test_tick_shutdown_requested() {
    let message = "{\"action\": \"shutdownRequested\", \"checkpoint\": \"1234\"}";
    let (processor, writer) = tick_into_processor(message);

    assert!(processor.shutdown_requested);
    assert_eq!(writer.outputs.len(), 1);
    assert_status_response(&writer, "shutdownRequested");
}
