pub mod error;
pub mod repository;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::{config::Builder, Client};

pub async fn connect(local: bool) -> Client {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let mut builder = Builder::from(&config);
    if local {
        builder.set_endpoint_url(Some("http://127.0.0.1:4566".into()));
        builder.set_region(Some(Region::from_static("us-east-1")));
    }

    Client::from_conf(builder.build())
}
