use anyhow::Result;
use crate::config::Config;

pub async fn bootstrap(_config: &Config) -> Result<()> {
    crate::storage::init().await?;
    crate::commands::registry::validate_registry();
    crate::protections::init().await?;
    crate::services::init().await?;
    Ok(())
}

/// Command routing contract.
///
/// The Rust version deliberately does not reproduce the 13k-line JavaScript
/// switch. Incoming WhatsApp messages will be normalized once and dispatched
/// through this registry, with permissions/protections applied before the
/// individual command handler.
pub fn normalize_command(text: &str, prefix: &str) -> Option<String> {
    let text = text.trim();
    if !text.starts_with(prefix) {
        return None;
    }
    let body = text[prefix.len()..].trim();
    let command = body.split_whitespace().next()?;
    Some(command.to_lowercase())
}
