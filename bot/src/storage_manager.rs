use aws_sdk_dynamodb::{
    error::{PutItemError, QueryError},
    model::AttributeValue,
    output::QueryOutput,
    types::SdkError,
    Client,
};
use serenity::model::id::UserId;

use crate::reactions::GamerResponseOption;

pub struct StorageManager {
    pub dynamo_client: Client,
    table_name: String,
}

pub struct PostReactionsDto {
    pub yes: Box<UserId>,
    pub maybe: Box<UserId>,
    pub late: Box<UserId>,
    pub no: Box<UserId>,
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
            .table_name(self.table_name.clone())
            .item("post_id", AttributeValue::S(post_id))
            .item("user_id", AttributeValue::S(user_id))
            .item("response", AttributeValue::S(reaction.to_string()));

        if let Err(error) = request.send().await {
            println!("Error on insert_reaction: {}", error);
            return Err(error);
        }

        Ok(())
    }

    pub async fn get_reactions_for_post(
        &self,
        post_id: String,
    ) -> Result<PostReactionsDto, SdkError<QueryError>> {
        match self
            .dynamo_client
            .query()
            .table_name(self.table_name.clone())
            .key_condition_expression("post_id = :post_id_value")
            .expression_attribute_values(":post_id_value", AttributeValue::S(post_id))
            .send()
            .await
        {
            Ok(result) => {
                let summary = Self::query_result_to_summary(result);
                Ok(summary)
            }
            Err(err) => {
                println!("Error querying for reactions: {}", &err);
                Err(err)
            }
        }
    }

    fn query_result_to_summary(query_output: QueryOutput) -> PostReactionsDto {
        let mut summary = PostReactionsDto {
            yes: Box::default(),
            maybe: Box::default(),
            late: Box::default(),
            no: Box::default(),
        };
        for item in query_output.items {
            for record in item {
                let post_id = record.get("post_id").unwrap().as_s().unwrap();
                let user_id = record.get("user_id").unwrap().as_s().unwrap();
                let response = record.get("response").unwrap().as_s().unwrap();
                println!("{:?}:{:?}:{:?}", post_id, user_id, response);
                // TODO: update the values in the summary
            }
        }
        summary
        // TODO: iterate over the query results and produce values for this
    }
}

pub async fn initialise(table_name: String) -> StorageManager {
    let config = aws_config::load_from_env().await;
    // let config = SdkConfig::builder().credentials_provider(SharedC).build();
    let client = Client::new(&config);
    StorageManager {
        dynamo_client: client,
        table_name,
    }
}
