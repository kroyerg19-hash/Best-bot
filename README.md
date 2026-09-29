# GIPSY RUST

Recriação modular do Gipsy em Rust, baseada no `index (5).js` fornecido.

## Importante

Este primeiro pacote contém a arquitetura, o registro de comandos e os contratos de migração. Os handlers ainda serão portados por etapas.

## Requisitos

- Rust 1.94+
- FFmpeg para funcionalidades de mídia quando forem portadas
- yt-dlp para downloads quando forem portados
- credenciais em variáveis de ambiente

## Variáveis

```text
GIPSY_CREATOR_NUMBER=...
GIPSY_PREFIX=!
GIPSY_DATA_DIR=./data
SPIDERX_API_KEY=...
GEMINI_API_KEY=...
OPENAI_API_KEY=...
TELEGRAM_BOT_TOKEN=...
RUST_LOG=info
```

Não coloque chaves reais no código-fonte.

## Próxima etapa

Implementar a conexão `whatsapp-rust`, recebimento de mensagens, normalização de JID/contexto, permissões e o primeiro lote de comandos.
