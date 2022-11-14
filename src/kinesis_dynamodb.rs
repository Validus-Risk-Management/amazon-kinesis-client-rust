use aws_lambda_events::dynamodb::UserIdentity;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(feature = "dynamodb-events")]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamoDBPayload {
    pub dynamodb: aws_lambda_events::dynamodb::StreamRecord,
    pub aws_region: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub event_name: String,
    pub user_identity: Option<UserIdentity>,
    pub record_format: String,
    pub table_name: String,
    pub event_source: String,
}

impl DynamoDBPayload {
    pub fn new_item<T: DeserializeOwned>(&self) -> serde_dynamo::Result<T> {
        serde_dynamo::from_item(self.dynamodb.clone().new_image)
    }
    pub fn old_item<T: DeserializeOwned>(&self) -> serde_dynamo::Result<T> {
        serde_dynamo::from_item(self.dynamodb.clone().old_image)
    }
}
