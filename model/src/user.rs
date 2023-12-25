use crate::error::ModelError;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use std::collections::HashMap;
use type_safe_id::{StaticType, TypeSafeId};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "pk")]
    pub id: String,
    #[serde(rename = "attr1")]
    pub name: String,
}

pub type UserID = TypeSafeId<User>;

impl StaticType for User {
    const TYPE: &'static str = "user";
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: UserID::new().to_string(),
            name,
        }
    }
}

impl TryFrom<User> for HashMap<String, AttributeValue> {
    type Error = ModelError;

    fn try_from(input: User) -> Result<Self, Self::Error> {
        let item = to_item(input).map_err(|e| ModelError {
            desc: format!("failed to convert user to item: {}", e),
        })?;

        Ok(item)
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for User {
    type Error = ModelError;

    fn try_from(input: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let user = serde_dynamo::from_item(input).map_err(|e| ModelError {
            desc: format!("failed to convert item to user: {}", e),
        })?;

        Ok(user)
    }
}
