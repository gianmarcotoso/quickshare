# apps/api

Crate `quickshare` (Rust 2024): Axum 0.8, tokio, utoipa per lo spec OpenAPI, rust-embed per la pagina, sha2 per gli hash, qrcode per il QR nel terminale. Struttura ricalcata su Chemistry, ridotta a ciò che serve.

```
src/
  main.rs                       # CLI o avvio del server
  app.rs, app/state.rs          # init (config + sessione) e start (listener, URL e QR, spegnimento); AppState
  cli.rs, cli/{init,openapi}.rs
  core/
    app_config.rs               # lettura di config.ini (facoltativo), API_PATH
    session.rs                  # token, inizio, hash dell'eseguibile, lotti aperti
    version.rs                  # VERSION = versione (commit), dal build script
    error_code.rs               # ErrorCode (ERR_*) ed ErrorDto
    action_error.rs             # errori delle azioni → risposta HTTP; internal(causa) logga la causa e restituisce ERR_INTERNAL
    logging.rs                  # tracing su stderr
  features/
    panel/                      # pagina inclusa nel binario
    uploads/                    # lotti e ricezione dei file
      actions/                  # una funzione per file
      controllers/, dtos/, types/, constants.rs
      libs/                     # funzioni pure usate solo da uploads
      middlewares/require_token.rs
      batch.rs, partial_file.rs # componenti con ciclo di vita (vedi sotto)
  libs/                         # funzioni pure generiche o condivise tra feature, una per file
  openapi.rs                    # ApiDoc
  router/                       # router, rotte /api
  tests/                        # test del router
```

## Endpoint

Tutti richiedono `Authorization: Bearer <token>` (middleware `require_token`, che risponde da sé `401` con `ERR_INVALID_TOKEN` perché è un controllo di solo trasporto), tranne `GET /api/openapi.json`.

I controller estraggono l'input e chiamano una sola azione. IP e user agent arrivano come extractor `Uploader` (`FromRequestParts`, da `ConnectInfo` e dall'header `User-Agent`).

| Endpoint                         | Effetto                                                                 |
| -------------------------------- | ----------------------------------------------------------------------- |
| `GET /api/session`               | `204` se il token è valido: la pagina lo usa per mostrare subito "Link non valido" |
| `POST /api/batches`              | crea il lotto (cartella e intestazione del registro), `201` con `BatchDto { id }` |
| `POST /api/batches/{id}/files`   | `multipart/form-data` con il campo `file`, un file per richiesta; `201` con `ReceivedFileDto` (`name` effettivo su disco, `size`, `sha256`) |
| `POST /api/batches/{id}/close`   | chiude il lotto e sigilla il registro con il suo hash, `204`            |

I lotti aperti stanno in memoria nella `Session`, indicizzati per id (il nome della cartella): un id che non è un lotto aperto dà `404`, quindi un id non diventa mai un percorso su disco.

## Componenti con ciclo di vita

Stanno nella radice della feature `uploads`, fuori da `types/`, perché possiedono risorse e comportamento e non sono semplici definizioni:

- `Batch`: cartella del lotto e registro aperto in append, con i contatori protetti da un mutex. Crea la cartella con un nome unico, scrive le voci, e alla chiusura scrive il piè di pagina e l'hash delle righe precedenti; dopo la chiusura rifiuta altre voci.
- `PartialFile`: guard sul file in scrittura che al `drop` lo cancella e annota il registro, se non è stato disattivato con `keep`.

## Ricezione di un file

`receive_file` trova il lotto aperto (`404` se non c'è) e il campo `file` del multipart (`400 ERR_FILE_MISSING` se manca), poi passa a `save_file`. Il file non passa dalla memoria: `write_field` legge il campo a chunk e li scrive su disco (con `BufWriter`) aggiornando nello stesso passaggio lo SHA-256, poi `sync_all`. Il file appena creato è protetto da un `PartialFile`: se la richiesta fallisce, o il future viene abbandonato (connessione caduta, spegnimento), al `drop` il file viene cancellato e il registro riceve la riga `upload not completed`; solo a voce registrata il guard viene disattivato. La voce del file si scrive nel registro sotto lo stesso lock che segna il lotto come chiuso, così nulla finisce dopo la riga con l'hash; un file che completa dopo la chiusura viene scartato. IP da `ConnectInfo` (IPv4 mappati su IPv6 normalizzati), dispositivo dallo user agent.

Il limite di dimensione (`Upload.MaxBytes`) è un `DefaultBodyLimit` sulla sola rotta di invio; con `0` il limite di Axum (2 MB) è disattivato. Con `Content-Length` noto e oltre il limite la richiesta è rifiutata prima di creare il file.

## Errori

Codici `ERR_*` in `core/error_code.rs`, corpo `{ "code": "ERR_..." }`:

| Codice                   | Status | Quando                                         |
| ------------------------ | ------ | ---------------------------------------------- |
| `ERR_INVALID_TOKEN`      | 401    | token assente o diverso da quello della sessione |
| `ERR_BATCH_NOT_FOUND`    | 404    | lotto inesistente o già chiuso                 |
| `ERR_FILE_MISSING`       | 400    | nessun campo `file` nella richiesta            |
| `ERR_UPLOAD_INTERRUPTED` | 400    | lo stream multipart si è interrotto            |
| `ERR_FILE_TOO_LARGE`     | 413    | il file supera `Upload.MaxBytes`               |
| `ERR_INTERNAL`           | 500    | errore di I/O (loggato come `error`)           |

## Pagina inclusa

`features/panel/controllers/panel.rs` include `apps/web/dist` con rust-embed (`allow_missing`: senza build della pagina il crate compila e `/` risponde `404 Panel not built`). In debug i file sono letti da disco a runtime, in release sono nel binario. Asset sotto `assets/` con `Cache-Control: immutable`, il resto `no-cache`; un path senza estensione non trovato restituisce `index.html`, un path sotto `/api` resta `404`. Servendo `index.html` il server inserisce `<meta name="quickshare-version">` con la versione completa (`VERSION`, con il commit).

## Test

`cargo test` in `apps/api`. I test del router creano un `AppState` (`app::init`, con un hash dell'eseguibile fittizio) su una cartella temporanea e chiamano il router con `tower::ServiceExt::oneshot`, con `MockConnectInfo` per l'IP (`tests/support.rs`).

- `router_uploads_flow` — token assente o errato, una cartella per lotto con il nome giusto, salvataggio con hash e voce di registro (intestazione con token e sessione, dimensione, dispositivo, IP), chiusura con hash verificabile delle righe precedenti e lotto non più utilizzabile, nomi duplicati numerati, nomi che tentano di uscire dalla cartella, lotti sconosciuti, richiesta senza file, file oltre il limite non conservato.
- `router_panel` — spec OpenAPI servito, `404` per percorsi API sconosciuti e asset mancanti.
- Unit test in `libs/` (anche `numbered_file_name`, i suffissi dei nomi duplicati) e in `PartialFile` (cancellazione e nota nel registro).
