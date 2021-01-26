use lambda_http::{handler, lambda, IntoResponse, Request, RequestExt, Context};
use lambda_http::http::{StatusCode, Response, HeaderValue};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use rusoto_core::{Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use rusoto_s3::{PutObjectRequest};
use rusoto_s3::util::PreSignedRequest;
use rusoto_credential::{ChainProvider, ProvideAwsCredentials};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize)]
struct RequestBody {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(cat_post)).await?;
    Ok(())
}

async fn cat_post(request: Request, _: Context) -> Result<impl IntoResponse, Error> {

    let body: RequestBody = match request.payload() {
        Ok(Some(body)) => body,
        _ => {return Ok(
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid payload".into())
                .expect("Failed to render response")
            )
        }
    };

    let client = DynamoDbClient::new(Region::EuCentral1);

    let mut new_cat = HashMap::new();
    new_cat.insert("name".to_string(), AttributeValue { s: Some(body.name.clone()), ..Default::default() });
    let image_path = format!("image/{}.jpg", &body.name);
    new_cat.insert("image_path".to_string(), AttributeValue { s: Some(image_path.clone()), ..Default::default() });

    let put_item_input = PutItemInput {
        table_name: "shing_catdex".to_string(),
        item: new_cat,
        ..Default::default()
    };

    match client.put_item(put_item_input).await {
        Ok(_)=> (),
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Something went wrong when writing to the database".into())
                .expect("Failed to render response"))
        }
    }

    let credentials = ChainProvider::new().credentials().await.unwrap();

    let put_request = PutObjectRequest {
        bucket: "shing-catdex-frontend".to_string(),
        key: image_path,
        content_type: Some("image/jpeg".to_string()),
        ..Default::default()
    };

    let presigned_url = put_request.get_presigned_url(&Region::EuCentral1, &credentials, &Default::default());

    let mut response = json!({"upload_url": presigned_url}).into_response();
    response.headers_mut().insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    Ok(response)
}
