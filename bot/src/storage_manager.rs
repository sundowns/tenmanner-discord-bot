use aws_sdk_dynamodb::{error::PutItemError, model::AttributeValue, types::SdkError, Client};

use crate::reactions::GamerResponseOption;

pub struct StorageManager {
    pub dynamo_client: Client,
}

impl StorageManager {
    // https://dynobase.dev/dynamodb-rust/#put-item
    pub async fn insert_reaction(
        &self,
        post_id: String,
        user_id: String,
        reaction: GamerResponseOption,
    ) -> Result<(), SdkError<PutItemError>> {
        let request = self
            .dynamo_client
            .put_item()
            .table_name("tenmannerreactions")
            .item("post_id", AttributeValue::S(post_id))
            .item("user_id", AttributeValue::S(user_id))
            .item("response", AttributeValue::S(reaction.to_string()));

        request.send().await?;

        Ok(())
    }
}

pub async fn login() -> StorageManager {
    let config = aws_config::load_from_env().await;
    // let config = SdkConfig::builder().credentials_provider(SharedC).build();
    let client = Client::new(&config);
    StorageManager {
        dynamo_client: client,
    }
}
