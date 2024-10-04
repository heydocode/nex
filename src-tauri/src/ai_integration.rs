// Error Handling
use thiserror::Error;

// genai API crate
use genai::Client;
use genai::adapter::AdapterKind;
use genai::chat::{ChatRequest, ChatMessage};

// Error Handling Definitions
#[derive(Debug, Error)]
pub enum AppError {
    #[error("GenAI API error: {0}")]
    GenAIError(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

const AI_PROVIDER: genai::adapter::AdapterKind = AdapterKind::Ollama;
const MODEL: &str = "llama3.1"; // Example constant, adjust based on your choice

async fn generate_response(text_prompt: &str) -> Result<String> {
    let client = Client::default(); // Initialize the genai client
    
    let chat_req = ChatRequest::new(vec![
        ChatMessage::user(text_prompt),
    ]);

    let response = client
        .exec_chat(MODEL, chat_req, None)
        .await
        .map_err(|e| AppError::GenAIError(e.to_string()))?;

    Ok(response.content_text_as_str().unwrap_or_default().to_string())
}

pub async fn send_ai_request(text_prompt: &str) -> Result<String> {
    // Generate a response from the AI model
    let response = generate_response(text_prompt).await?;
    Ok(response)
}

pub async fn test_ai_availability() -> bool {
    let client = Client::default();

    // Retrieve model names for the Ollama AdapterKind
    let models = match client.all_model_names(AI_PROVIDER).await {
        Ok(models) => models,
        Err(_) => {
            #[cfg(debug_assertions)]
            println!("Failed to retrieve models.");
            return false;
        }
    };

    let model_available = models.iter().any(|model| model == MODEL);

    if !model_available {
        #[cfg(debug_assertions)]
        println!("Model '{}' isn't available. Available AI models: \n{:?}", MODEL, models);
    }

    model_available
}
