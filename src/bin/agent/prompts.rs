use anyhow::Result;
use chatgpt::{
    prelude::{ChatGPT, ModelConfigurationBuilder},
    types::CompletionResponse,
};

pub async fn generate_persona(openai_api_key: &str) -> Result<String> {
    // TODO: generate big list of jobs, hobbies, age etc and randomly select from them
    let client = ChatGPT::new_with_config(
        openai_api_key,
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
