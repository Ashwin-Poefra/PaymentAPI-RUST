use async_once::AsyncOnce;
use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_dynamodb::{Client, Error, Region};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        return make_client().await.expect("Failed to initialize DynamoDB connection");
    });
}

pub async fn make_client() -> Result<Client, Error> {
    let config = make_config(None).await?;
    let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();

    let client = Client::from_conf(dynamodb_config);
    return Ok(client);
}

fn make_region_provider(region: Option<String>) -> RegionProviderChain {
    RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"))
}

async fn make_config(region: Option<String>) -> Result<SdkConfig, Error> {
    let region_provider = make_region_provider(region);

    return Ok(aws_config::from_env().region(region_provider).load().await);
}
