use anyhow::Result;
use chatgpt::{
    prelude::{ChatGPT, ModelConfigurationBuilder},
    types::CompletionResponse,
};
use common::structs_api::ApiFrontpage;
use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let service_url = std::env::var("SERVICE_URL").unwrap();

    // print frontpage
    let frontpage = get_frontpage(&service_url).await?;
    println!("{:?}", frontpage);

    println!("Generating persona...");
    let persona = generate_persona(&openai_api_key).await?;

    println!("Simulating persona...");
    let prompt = simulation_prompt(&persona, frontpage).await;
    println!("{}", prompt);




// ... within your conversation, before sending first message
    let client = ChatGPT::new_with_config(
        openai_api_key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;
    let mut conversation = client.new_conversation();
    // note that you need to call the function when adding it
    // conversation.add_function(explain_intent())?;
    conversation.add_function(view_post())?;
    let response = conversation
        .send_message_functions(prompt)
        .await?;

    println!("{}", response.message().content);





    // generate 5 personas:
    // for _ in 0..5 {
    //     let persona = generate_persona(&openai_api_key).await?;
    //     println!("{}\n", persona);
    // }
    Ok(())
}


/// Give an explanation for what and why you are doing something
/// 
/// * explanation - brief explanation
#[gpt_function]
async fn explain_intent(explanation: String) {
    println!("explanation: {explanation}");
}

/// Click a post to view it's details, including discussion
/// 
/// * explanation - explanation of why you are viewing this post
/// * post_id - id of the post to view
#[gpt_function]
async fn view_post(explanation: String, post_id: String) {
    println!("Viewing post. {explanation} post_id: {post_id}");
}


pub async fn simulation_prompt(persona: &str, frontpage: ApiFrontpage) -> String {
    let frontpage_json = serde_json::to_string_pretty(&frontpage).unwrap();
    format!(r#"
    You are the following persona:
    ```
    {persona}
    ```

    You are using a social media platform, like Twitter. You engage with things that interest you, raise your opinions and vote on things you like. 

    You are looking at the frontpage:
    ```
    {frontpage_json}
    ```
    Possible actions:
    - View a post

    Now call a function.

    "#)
}

pub async fn get_frontpage(service_url: &str) -> Result<ApiFrontpage> {
    let client = reqwest::Client::new();
    let url = format!("{service_url}/api/v0/frontpage");
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<ApiFrontpage>()
        .await?;
    Ok(response)
}

pub async fn generate_persona(key: &str) -> Result<String> {
    // TODO: generate big list of jobs, hobbies, age etc and randomly select from them
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;

    let prompt =
        r#"concisely generate a random persona. Start with name, age, personality traits, occupation, interests. No explanations."#.to_string();

    let response: CompletionResponse = client.send_message(prompt).await?;
    Ok(response.message().content.clone())
}
