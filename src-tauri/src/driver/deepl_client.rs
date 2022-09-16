use anyhow::bail;
use error_chain::*;
use serde::Deserialize;

use crate::config::SETTINGS;

#[mry::mry]
pub async fn translate_to_ja(query: &Vec<(String, String)>) -> anyhow::Result<TranslatedTextList> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let client = reqwest::Client::new();

    let payload = query.clone();

    let res = match client
        .post(&url)
        .header(
            "Authorization",
            format!("DeepL-Auth-Key {}", SETTINGS.deepl_token.clone()),
        )
        .query(&payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => response,
        Ok(response) if response.status() == reqwest::StatusCode::UNAUTHORIZED => {
            bail!(ErrorKind::AuthorizationError)
        }
        Ok(response) if response.status() == reqwest::StatusCode::FORBIDDEN => {
            bail!(ErrorKind::AuthorizationError)
        }

        Ok(response) => {
            let status = response.status();
            match response.json::<ServerErrorMessage>().await {
                Ok(server_error) => bail!(ErrorKind::ServerError(server_error.message)),
                _ => bail!(ErrorKind::ServerError(status.to_string())),
            }
        }
        Err(e) => {
            bail!(e)
        }
    };

    Ok(res.json::<TranslatedTextList>().await?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranslatedTextList {
    pub translations: Vec<TranslatedText>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TranslatedText {
    pub detected_source_language: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerErrorMessage {
    pub message: String,
}

mod errors {
    use error_chain::*;
    error_chain! {}
}

pub use errors::*;

error_chain! {
    foreign_links {
        IO(std::io::Error);
        Transport(reqwest::Error);
    }
    errors {
        /// Indicates that the provided API key was refused by the DeepL server.
        AuthorizationError {
            description("Authorization failed, is your API key correct?")
            display("Authorization failed, is your API key correct?")
        }
        /// An error occurred on the server side when processing a request. If possible, details
        /// will be provided in the error message.
        ServerError(message: String) {
            description("An error occurred while communicating with the DeepL server.")
            display("An error occurred while communicating with the DeepL server: '{}'.", message)
        }
        /// An error occurred on the client side when deserializing the response data.
        DeserializationError {
            description("An error occurred while deserializing the response data.")
            display("An error occurred while deserializing the response data.")
        }
    }

    skip_msg_variant
}
