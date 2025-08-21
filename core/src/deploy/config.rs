use aws_config::{
    BehaviorVersion, Region, SdkConfig, meta::region::RegionProviderChain, retry::RetryConfig,
};

const DEFAULT_REGION: &str = "eu-west-1"; // fuck us-east-1

pub(crate) async fn load(region: &Option<String>, retry: &u32) -> anyhow::Result<SdkConfig> {
    let region = RegionProviderChain::first_try(region.clone().map(|r| Region::new(r)))
        .or_default_provider()
        .or_else(Region::new(DEFAULT_REGION));

    let retry = RetryConfig::standard().with_max_attempts(*retry);

    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .retry_config(retry)
        .load()
        .await;

    Ok(cfg)
}
