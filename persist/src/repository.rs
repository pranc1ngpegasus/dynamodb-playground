use crate::error::PersistError;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

#[async_trait]
pub trait Repository {
    async fn put_item(
        &self,
        table_name: String,
        input: HashMap<String, AttributeValue>,
    ) -> Result<(), PersistError>;
    async fn get_item(
        &self,
        table_name: String,
        pk: String,
        sk: String,
    ) -> Result<HashMap<String, AttributeValue>, PersistError>;
}

#[derive(Clone)]
pub struct RepositoryImpl {
    pub client: Client,
}

impl RepositoryImpl {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Repository for RepositoryImpl {
    async fn put_item(
        &self,
        table_name: String,
        input: HashMap<String, AttributeValue>,
    ) -> Result<(), PersistError> {
        self.client
            .put_item()
            .table_name(table_name)
            .set_item(Some(input))
            .send()
            .await
            .map_err(|e| PersistError {
                desc: format!("failed to insert item: {}", e.into_service_error().meta()),
            })?;

        Ok(())
    }

    async fn get_item(
        &self,
        table_name: String,
        pk: String,
        sk: String,
    ) -> Result<HashMap<String, AttributeValue>, PersistError> {
        let resp = self
            .client
            .get_item()
            .table_name(table_name)
            .key("pk", AttributeValue::S(pk))
            .key("sk", AttributeValue::S(sk))
            .send()
            .await
            .map_err(|e| PersistError {
                desc: format!("failed to query item: {}", e.into_service_error().meta()),
            })?;

        match resp.item {
            Some(item) => Ok(item),
            None => Err(PersistError {
                desc: "item not found".to_string(),
            }),
        }
    }
}
