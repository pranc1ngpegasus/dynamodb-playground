use crate::error::ModelError;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use std::collections::HashMap;
use type_safe_id::{StaticType, TypeSafeId};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "pk")]
    pub id: String,
    #[serde(rename = "attr1")]
    pub user_ids: Vec<String>,
}

pub type GroupID = TypeSafeId<Group>;

impl StaticType for Group {
    const TYPE: &'static str = "group";
}

impl Group {
    pub fn new(user_ids: Vec<String>) -> Self {
        Self {
            id: GroupID::new().to_string(),
            user_ids,
        }
    }
}

impl TryFrom<Group> for HashMap<String, AttributeValue> {
    type Error = ModelError;

    fn try_from(input: Group) -> Result<Self, Self::Error> {
        let item = to_item(input).map_err(|e| ModelError {
            desc: format!("failed to convert group to item: {}", e),
        })?;

        Ok(item)
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Group {
    type Error = ModelError;

    fn try_from(input: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let user = serde_dynamo::from_item(input).map_err(|e| ModelError {
            desc: format!("failed to convert item to group: {}", e),
        })?;

        Ok(user)
    }
}
