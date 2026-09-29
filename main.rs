mod config;
mod commands;
mod router;
mod storage;
mod protections;
mod services;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "gipsy_rust=info,whatsapp_rust=info".into()),
        )
        .init();

    let config = config::Config::from_env()?;
    info!("GIPSY RUST iniciando...");
    info!("Criador configurado: {}", config.creator_number);

    // A integração concreta com whatsapp-rust será ligada aqui.
    // Mantemos o roteador e os módulos independentes para que a migração
    // dos comandos possa acontecer sem concentrar tudo em main.rs.
    router::bootstrap(&config).await?;

    tokio::signal::ctrl_c().await?;
    info!("GIPSY RUST encerrando.");
    Ok(())
}
