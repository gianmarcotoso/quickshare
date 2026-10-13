# CLAUDE.md

QuickShare — un binario Rust che serve una pagina web da cui un cliente, collegato alla stessa rete (tipicamente l'hotspot del laptop), invia uno o più file. Stesso stack e stesse convenzioni di Chemistry (`../chemistry`). Principio guida: **less is more** — una sola modalità per fare una cosa, niente funzionalità speculative.

Documentazione (stato attuale, da leggere prima di lavorare nell'area relativa):

- [docs/architecture.md](docs/architecture.md) — monorepo, configurazione, layout su disco, routing, log, CLI, build.
- [docs/api.md](docs/api.md) — `apps/api`: moduli, endpoint, ricezione dei file, test.
- [docs/web.md](docs/web.md) — `apps/web`: struttura, coda di invio, hash nel browser, sviluppo.
- [docs/ledger.md](docs/ledger.md) — storia delle decisioni, cronologica.

## Regole

- Documentazione e ledger in italiano; identificatori nel codice in inglese.
- Niente commenti nel codice salvo vincoli nascosti non deducibili; il "perché" va in `docs/`, la storia in `docs/ledger.md`.
- Dopo ogni modifica a endpoint o DTO: aggiornare `#[utoipa::path]` e `openapi.rs`, poi `npm run api-gen`. Mai modificare a mano `apps/web/src/api/`.
- Errori dal server solo come codici `ERR_*` (`core/error_code.rs`), mai testi; ogni codice nuovo va tradotto in `apps/web/src/resources/locales/*.json` (`errors.*`).
- Verifiche: `cargo test` e `cargo clippy --all-targets` in `apps/api`; `npx tsc -b` e `npm run lint` in `apps/web`.
