use anyhow::ensure;
use anyhow::Context;
use anyhow::Result;
use common::structs_api::{ApiCreatePost, ApiFrontpage, ApiPostPage, ApiVote};

pub async fn get_frontpage(service_url: &str) -> Result<ApiFrontpage> {
    let client = reqwest::Client::new();
    let response = client
        .get(&(format!("{service_url}/api/v0/frontpage")))
        .send()
        .await?;

    let result = successful_json::<ApiFrontpage>(response).await?;

    Ok(result)
}

pub async fn get_post_page(service_url: &str, post_id: i64) -> Result<ApiPostPage> {
    let client = reqwest::Client::new();
    let response = client
        .get(&(format!("{service_url}/api/v0/view_post/{post_id}")))
        .send()
        .await?;

    let result = successful_json::<ApiPostPage>(response).await?;

    Ok(result)
}

pub async fn vote_post(service_url: &str, vote: ApiVote) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post(&(format!("{service_url}/api/v0/vote")))
        .json(&vote)
        .send()
        .await?;

    response.error_for_status()?;
    Ok(())
}

pub async fn create_post(service_url: &str, create_post: ApiCreatePost) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post(&(format!("{service_url}/api/v0/create_post")))
        .json(&create_post)
        .send()
        .await?;

    response.error_for_status()?;
    Ok(())
}

pub async fn successful_json<T>(response: reqwest::Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    ensure!(
        response.status().is_success(),
        "Request failed: {}\n{}",
        response.status(),
        response.text().await?
    );

    let response_text = response.text().await?;
    let result = serde_json::from_str::<T>(&response_text);
    result.with_context(|| format!("json: {response_text}"))
}
