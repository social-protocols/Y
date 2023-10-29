mod api;
mod prompts;

use anyhow::Result;
use chatgpt::{
    functions::FunctionCall,
    prelude::{gpt_function, ChatGPT, ModelConfigurationBuilder},
    types::{ChatMessage, Role},
};

use common::structs_api::{ApiCreatePost, ApiDirection, ApiVote};

#[tokio::main]
async fn main() -> Result<()> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let service_url = std::env::var("SERVICE_URL").unwrap();

    let client = ChatGPT::new_with_config(
        &openai_api_key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;

    let mut conversation = client.new_conversation();
    conversation.add_function(view_frontpage())?;
    conversation.add_function(view_post())?;
    // conversation.add_function(vote_post())?;
    // conversation.add_function(reply_post())?;
    let persona = prompts::generate_persona(&openai_api_key).await?;
    // conversation.history.push(ChatMessage {
    //     role: Role::System,
    //     content: format!(
    //         r#"
    // You are the following persona:
    // ```
    // {persona}
    // ```
    //
    // - You are using a social media platform, like Twitter.
    // - You engage with things that interest you, raise your opinions and vote on things you like.
    // - Always call functions to act.
    //
    //
    // "#
    //     ),
    //     function_call: None,
    // });
    // conversation.history.push(ChatMessage {
    //     role: Role::Function,
    //     content: "".to_string(), // TODO, how does a function call look like?
    //     function_call: Some(FunctionCall {
    //         name: "view_frontpage".to_string(),
    //         arguments: r#"{"explanation": "I'd like to see what others are talking about."}"#
    //             .to_string(),
    //     }),
    // });

    // let response = conversation.send_history().await?;
    let response = conversation
        .send_message_functions(
            "start by looking at the frontpage and tell me, which posts there are. call a function to do that.",
        )
        .await?;

    for message in conversation.history.iter().enumerate() {
        println!("{message:#?}")
    }

    Ok(())
}

/// Visit the front page
///
/// * explanation - explanation of why you are viewing the front page
#[gpt_function]
pub async fn view_frontpage(explanation: String) -> String {
    let service_url = std::env::var("SERVICE_URL").unwrap();
    let frontpage = api::get_frontpage(&service_url).await.unwrap(); // TODO: proper error propagation
    let frontpage_json = serde_json::to_string_pretty(&frontpage).unwrap();

    format!(
        r#"
        ```
        {frontpage_json}
        ```
        "#
    )
}

/// Click a post to view it's details, including discussion
///
/// * explanation - explanation of why you are viewing this post
/// * post_id - id of the post to view
#[gpt_function]
pub async fn view_post(explanation: String, post_id: i64) -> String {
    println!("Viewing post. {explanation} post_id: {post_id}");
    let service_url = std::env::var("SERVICE_URL").unwrap();
    let post_page = api::get_post_page(&service_url, post_id).await.unwrap(); // TODO: proper error propagation
    let post_page_json = serde_json::to_string_pretty(&post_page).unwrap();

    format!(
        r#"
        ```
        {post_page_json}
        ```
        "#
    )
}

/// Vote on the currently viewed post. To vote on replies you have to view them first.
///
/// * explanation - explanation of why you are voting on this post
/// * post_id - id of the post
/// * note_id - if post is shown with a note, specify it
/// * direction - upvote or downvote
#[gpt_function]
pub async fn vote(
    explanation: String,
    post_id: i64,
    note_id: Option<i64>,
    direction: ApiDirection,
) {
    println!("Voting on post. {explanation} post_id: {post_id}, note_id: {note_id:?}, direction: {direction:?}");
    let service_url = std::env::var("SERVICE_URL").unwrap();
    api::vote_post(
        &service_url,
        ApiVote {
            post_id,
            note_id,
            direction,
        },
    )
    .await
    .unwrap(); // TODO: proper error propagation
}

/// Reply to a post
///
/// * explanation - explanation of why you are replying
/// * post_id - id of the post
/// * content - your reply text
#[gpt_function]
pub async fn reply(explanation: String, post_id: i64, content: String) {
    println!("Reploying to post. {explanation} post_id: {post_id}, content: {content}");
    let service_url = std::env::var("SERVICE_URL").unwrap();
    api::create_post(
        &service_url,
        ApiCreatePost {
            parent_id: Some(post_id),
            content,
        },
    )
    .await
    .unwrap(); // TODO: proper error propagation
}
