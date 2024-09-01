// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ollama_integration;
use ollama_integration::{send_ollama_request, test_ai_availability};

/// The level of logging information for log() function
enum InfoLevel {
    INFO,
    WARNING,
    ERROR
}

impl InfoLevel {
    fn to_enum(id: u8) -> Self {
        match id {
            0 => InfoLevel::INFO,
            1 => InfoLevel::WARNING,
            _ => InfoLevel::ERROR,
        }
    }

    fn prefix(self) -> String {
        match self {
            InfoLevel::INFO => String::from("<<|INFO|>>"),
            InfoLevel::WARNING => String::from("<<|WARN|>>"),
            InfoLevel::ERROR => String::from("<<|ERR|>>"),
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn send_prompt(history: &str, prompt: &str) -> Result<String, String> {
    match send_ollama_request(history, prompt).await {
        Ok(n) => Ok(n),
        _ => Err(String::from("Error while tried to send a request")) 
    }
}

#[tauri::command]
async fn test_backend() -> Result<String, String> {
    Ok(String::from("connected")) 
}
 
#[tauri::command]
async fn ai_status() -> bool {
    test_ai_availability().await
}

#[tauri::command]
async fn log(info_level: u8, message: &str) -> Result<String, ()> {
    let level = InfoLevel::to_enum(info_level);
    let log_msg = format!("{} {}", InfoLevel::prefix(level), message);
    println!("{log_msg}");
    Ok(String::from("success"))
}
 
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_prompt, test_backend, ai_status, log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");  
}