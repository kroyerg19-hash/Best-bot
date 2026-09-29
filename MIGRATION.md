# Plano de migração do Gipsy para Rust

## Fonte de verdade

A referência funcional é o `index (5).js` fornecido nesta conversa. A versão recuperada pelo Files possui 13.885 linhas indexadas e contém o registro central de comandos e o switch principal.

## Arquitetura alvo

- `whatsapp-rust`: transporte/protocolo WhatsApp.
- `router`: normalização e despacho.
- `commands`: handlers individuais.
- `protections`: pipeline central de proteção.
- `storage`: persistência consolidada em SQLite.
- `services`: SpiderX, Gemini, OpenAI, yt-dlp/FFmpeg, Telegram e utilidades.
- `economy`: Gold, banco, inventário, mercado negro, VIP, presentes e XP.
- `games`: jogos e interações.
- `media`: stickers, áudio, vídeo, downloads e conversões.
- `ai`: Gipsy AI, memória, contexto e geração/edição de imagem.
- `telegram`: vínculo e migração de figurinhas.

## Proteções que precisam ser preservadas

1. Anti-link
2. Anti-fantasma
3. Anti-pagamento
4. Anti-flood
5. Anti-bot
6. Anti-fake
7. Anti-sticker
8. Anti-áudio
9. Anti-vídeo
10. Anti-documento
11. Anti-foto
12. Anti-mention
13. Anti-contato
14. Anti-location
15. Anti-palavra
16. Anti-roubo

## Anti-fantasma

A implementação atual possui um detector central que cruza a estrutura de pagamento com o rastreamento de eventos recebidos. A migração deve preservar essa ideia, incluindo `requestPaymentMessage`, `sendPaymentMessage`, `paymentMessage`, mensagens citadas, `stanzaId`, participante, janela de ativação e exclusões para bot/criador/admin.

## Dados persistentes

A versão JS mantém mapas em vários arquivos JSON, incluindo níveis, Gold, banco, inventário, mercado negro, casamentos, configurações, proteções, advertências, blacklist, VIP, presentes, divmsg e memória/contexto do Gipsy.

No Rust, a proposta é consolidar esses dados em SQLite com tabelas separadas e migrações, em vez de manter dezenas de arquivos JSON.

## Segurança de credenciais

As chaves que aparecem hardcoded no JavaScript **não serão copiadas para o Rust**. O projeto usa variáveis de ambiente (`SPIDERX_API_KEY`, `GEMINI_API_KEY`, `OPENAI_API_KEY`, `TELEGRAM_BOT_TOKEN`, etc.).

## Estratégia

Não fazer uma conversão mecânica do `index.js`. Primeiro o núcleo WhatsApp + roteador + persistência; depois proteções; depois comandos por módulos; por fim mídia, IA e Telegram.

## Estado deste pacote

Este pacote é o **esqueleto inicial da migração**, não uma alegação de que todos os comandos já foram portados.
