// Real LLM Integration Example
// This file shows how to actually integrate with OpenAI's API or Anthropic's Claude API

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use tokio;

// ============================================================================
// OpenAI API Integration
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: Option<serde_json::Value>,
}

// OpenAI API Client
pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        OpenAIClient {
            client: Client::new(),
            api_key,
        }
    }
    
    pub async fn chat_completion(
        &self,
        messages: Vec<OpenAIMessage>,
        model: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let request = OpenAIRequest {
            model: model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
            messages,
            max_tokens: Some(500),
            temperature: Some(0.7),
        };
        
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }
        
        let openai_response: OpenAIResponse = response.json().await?;
        
        openai_response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| "No response from OpenAI".into())
    }
}

// ============================================================================
// Anthropic Claude API Integration
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    usage: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

// Anthropic Claude API Client
pub struct ClaudeClient {
    client: Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        ClaudeClient {
            client: Client::new(),
            api_key,
        }
    }
    
    pub async fn chat_completion(
        &self,
        messages: Vec<ClaudeMessage>,
        model: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let request = ClaudeRequest {
            model: model.unwrap_or_else(|| "claude-3-sonnet-20240229".to_string()),
            max_tokens: 500,
            messages,
            temperature: Some(0.7),
        };
        
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }
        
        let claude_response: ClaudeResponse = response.json().await?;
        
        claude_response
            .content
            .into_iter()
            .find(|content| content.content_type == "text")
            .and_then(|content| content.text)
            .ok_or_else(|| "No text response from Claude".into())
    }
}

// ============================================================================
// High-Level AI Assistant Interface
// ============================================================================

pub enum AIProvider {
    OpenAI(OpenAIClient),
    Claude(ClaudeClient),
}

pub struct AIAssistant {
    provider: AIProvider,
}

impl AIAssistant {
    pub fn new_openai(api_key: String) -> Self {
        AIAssistant {
            provider: AIProvider::OpenAI(OpenAIClient::new(api_key)),
        }
    }
    
    pub fn new_claude(api_key: String) -> Self {
        AIAssistant {
            provider: AIProvider::Claude(ClaudeClient::new(api_key)),
        }
    }
    
    pub async fn ask_about_rust(&self, topic: &str) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            "Explain this Rust programming concept clearly and concisely with examples: {}",
            topic
        );
        
        match &self.provider {
            AIProvider::OpenAI(client) => {
                let messages = vec![
                    OpenAIMessage {
                        role: "system".to_string(),
                        content: "You are an expert Rust programmer who explains concepts clearly with practical examples.".to_string(),
                    },
                    OpenAIMessage {
                        role: "user".to_string(),
                        content: prompt,
                    },
                ];
                client.chat_completion(messages, None).await
            }
            AIProvider::Claude(client) => {
                let messages = vec![
                    ClaudeMessage {
                        role: "user".to_string(),
                        content: format!("You are an expert Rust programmer. {}", prompt),
                    },
                ];
                client.chat_completion(messages, None).await
            }
        }
    }
    
    pub async fn debug_rust_code(&self, code: &str, error: &str) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            "Help debug this Rust code. Code:\n```rust\n{}\n```\nError: {}\n\nPlease explain the issue and provide a fix.",
            code, error
        );
        
        match &self.provider {
            AIProvider::OpenAI(client) => {
                let messages = vec![
                    OpenAIMessage {
                        role: "system".to_string(),
                        content: "You are a Rust expert who helps debug code. Provide clear explanations and corrected code.".to_string(),
                    },
                    OpenAIMessage {
                        role: "user".to_string(),
                        content: prompt,
                    },
                ];
                client.chat_completion(messages, None).await
            }
            AIProvider::Claude(client) => {
                let messages = vec![
                    ClaudeMessage {
                        role: "user".to_string(),
                        content: format!("You are a Rust debugging expert. {}", prompt),
                    },
                ];
                client.chat_completion(messages, None).await
            }
        }
    }
    
    pub async fn generate_rust_code(&self, description: &str) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            "Generate Rust code for the following requirement: {}\n\nPlease provide clean, idiomatic Rust code with comments.",
            description
        );
        
        match &self.provider {
            AIProvider::OpenAI(client) => {
                let messages = vec![
                    OpenAIMessage {
                        role: "system".to_string(),
                        content: "You are a Rust expert who writes clean, idiomatic code. Always include proper error handling and comments.".to_string(),
                    },
                    OpenAIMessage {
                        role: "user".to_string(),
                        content: prompt,
                    },
                ];
                client.chat_completion(messages, None).await
            }
            AIProvider::Claude(client) => {
                let messages = vec![
                    ClaudeMessage {
                        role: "user".to_string(),
                        content: format!("You are a Rust code generation expert. {}", prompt),
                    },
                ];
                client.chat_completion(messages, None).await
            }
        }
    }
}

// ============================================================================
// Usage Examples
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example usage - you would need to provide actual API keys
    let api_key = std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .unwrap_or_else(|_| "your-api-key-here".to_string());
    
    // Create AI assistant (try OpenAI first, fallback to Claude)
    let assistant = if std::env::var("OPENAI_API_KEY").is_ok() {
        AIAssistant::new_openai(api_key)
    } else {
        AIAssistant::new_claude(api_key)
    };
    
    // Example 1: Ask about Rust concepts
    println!("🤖 AI Assistant Demo - Rust Concepts");
    
    let topics = vec!["ownership", "borrowing", "lifetimes", "async/await"];
    
    for topic in topics {
        println!("\n📚 Topic: {}", topic);
        match assistant.ask_about_rust(topic).await {
            Ok(response) => println!("AI: {}", response),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    // Example 2: Debug Rust code
    println!("\n🐛 AI Assistant Demo - Code Debugging");
    
    let buggy_code = r#"
fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s);
}
"#;
    
    let error = "borrow of moved value: `s`";
    
    match assistant.debug_rust_code(buggy_code, error).await {
        Ok(response) => println!("AI Debug Help: {}", response),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 3: Generate Rust code
    println!("\n🔧 AI Assistant Demo - Code Generation");
    
    let requirement = "Create a thread-safe counter that can be incremented from multiple threads";
    
    match assistant.generate_rust_code(requirement).await {
        Ok(response) => println!("AI Generated Code: {}", response),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}

// ============================================================================
// Configuration and Environment Setup
// ============================================================================

pub struct AIConfig {
    pub openai_api_key: Option<String>,
    pub claude_api_key: Option<String>,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl AIConfig {
    pub fn from_env() -> Self {
        AIConfig {
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            claude_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            default_model: std::env::var("DEFAULT_AI_MODEL")
                .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
            max_tokens: std::env::var("MAX_TOKENS")
                .unwrap_or_else(|_| "500".to_string())
                .parse()
                .unwrap_or(500),
            temperature: std::env::var("TEMPERATURE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()
                .unwrap_or(0.7),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_config_from_env() {
        let config = AIConfig::from_env();
        assert!(config.max_tokens > 0);
        assert!(config.temperature >= 0.0 && config.temperature <= 1.0);
    }
    
    #[tokio::test]
    async fn test_mock_ai_assistant() {
        // This would be a mock test - in real scenarios you'd use a test server
        // or mock the HTTP client
        let config = AIConfig::from_env();
        assert!(config.default_model.len() > 0);
    }
}

// ============================================================================
// Error Types for Better Error Handling
// ============================================================================

#[derive(Debug)]
pub enum AIError {
    NetworkError(String),
    ApiError(String),
    ParseError(String),
    ConfigError(String),
}

impl std::fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AIError::ApiError(msg) => write!(f, "API error: {}", msg),
            AIError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AIError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AIError {}

// ============================================================================
// Utility Functions
// ============================================================================

pub fn setup_logging() {
    env_logger::init();
}

pub fn load_config_from_file(path: &str) -> Result<AIConfig, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: AIConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_conversation(messages: &[String], filename: &str) -> Result<(), Box<dyn Error>> {
    let content = messages.join("\n\n---\n\n");
    std::fs::write(filename, content)?;
    Ok(())
}
