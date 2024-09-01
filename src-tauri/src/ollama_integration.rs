// Error Handling
use thiserror::Error;

// Ollama API crate
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

// Error Handling Definitions
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Ollama API error: {0}")]
    OllamaError(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

const MODEL: &str = "llama3.1:latest";

async fn generate_response(text_prompt: &str) -> Result<String> {
    let ollama = Ollama::default();
    let request = GenerationRequest::new(MODEL.to_string(), text_prompt.to_string());
    let response = ollama.generate(request).await.map_err(|e| AppError::OllamaError(e.to_string()))?;
    Ok(response.response)
}

pub async fn send_ollama_request(history: &str, text_prompt: &str) -> Result<String> {
    // Generate a response from the AI model
    let prompt = &format!("History: ###\"{}\"### | Current prompt: ###\"{}\"###", history, text_prompt);
    let response = generate_response(prompt).await?;

    println!("Got response \"{}\"", response);

    Ok(response)
}

pub async fn test_ai_availability() -> bool {
    let ollama = Ollama::default();
    
    let ai_list = match ollama.list_local_models().await {
        Ok(models) => models,
        Err(_) => {
            println!("Ollama is not running or failed to retrieve models.");
            return false;
        }
    };

    let nex_ai_available = ai_list.iter().any(|model| model.name == MODEL);
    
    if !nex_ai_available {
        println!("Nex isn't available. Available AI models: \n{:?}", ai_list);
    }
    
    nex_ai_available
}
