mod mocks;

use crate::mocks::mock_processor::MockCheckpointingProcessor;
use crate::mocks::mock_reader::MockReader;
use crate::mocks::mock_writer::MockWriter;
use kcl::tick;

fn tick_into_processor(
    message: &str,
    checkpoint_response: &str,
) -> (MockCheckpointingProcessor, MockWriter) {
    let mut processor = MockCheckpointingProcessor::default();
    let mut reader = MockReader::with_input(message.to_string());
    reader.add_input(checkpoint_response.to_string());

    let mut writer = MockWriter::default();

    tick(&mut processor, &mut reader, &mut writer).unwrap();

    (processor, writer)
}

#[test]
fn test_checkpoint_success() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response = "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":null}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    assert_eq!(writer.outputs.len(), 2);
    // Last thing sent is process records status
    let expected_process_records_status =
        "{\"action\":\"status\",\"responseFor\":\"processRecords\"}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_process_records_status.to_string())
    );

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "InvalidStateException, retryable: false")]
fn test_checkpoint_invalid_state_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response =
        "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":\"InvalidStateException\"}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "KinesisClientLibDependencyException, retryable: true")]
fn test_checkpoint_kcl_dep_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response =
        "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":\"KinesisClientLibDependencyException\"}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "ThrottlingException, retryable: true")]
fn test_checkpoint_throttle_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response =
        "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":\"ThrottlingException\"}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "ShutdownException, retryable: false")]
fn test_checkpoint_shutdown_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response =
        "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":\"ShutdownException\"}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "Exception: \"check your point\", retryable: false")]
fn test_checkpoint_unknown_error_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response =
        "{\"action\":\"checkpoint\",\"checkpoint\":null,\"error\":\"check your point\"}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}

#[test]
#[should_panic(expected = "UnexpectedResponse, retryable: false")]
fn test_checkpoint_unexpected_response_exception() {
    let message = "{\"action\" :\"processRecords\", \"records\": []}";
    let checkpoint_response = "{\"action\" :\"processRecords\", \"records\": []}";
    let (_processor, mut writer) = tick_into_processor(message, checkpoint_response);

    // First thing sent should be request to begin checkpoint
    let expected_checkpoint_status =
        "{\"action\":\"checkpoint\",\"sequenceNumber\":null,\"subSequenceNumber\":null}\n";
    assert_eq!(
        writer.outputs.pop(),
        Some(expected_checkpoint_status.to_string())
    );
}
