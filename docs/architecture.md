# Architettura

QuickShare riceve file da un browser sulla stessa rete. Un'istanza è un binario (pagina inclusa), un `config.ini` facoltativo e una cartella dati. Caso d'uso: il laptop crea un hotspot, QuickShare ascolta sull'IP dell'hotspot, il cliente apre l'URL (o inquadra il QR code) mostrato nel terminale, sceglie i file e preme Carica.

## Monorepo

Workspace npm (`apps/*`), come Chemistry:

- `apps/api` — il crate Rust `quickshare`: server HTTP (Axum), CLI. Vedi [api.md](api.md).
- `apps/web` — la pagina di invio (Vite + React + TypeScript + shadcn + Tailwind). Il build finisce in `apps/web/dist`, incluso nel binario. Vedi [web.md](web.md).

| Comando           | Effetto                                                              |
| ----------------- | -------------------------------------------------------------------- |
| `npm run dev:api` | `cargo run` in `apps/api` (usa `apps/api/config.ini`, se esiste)     |
| `npm run dev:web` | dev server Vite su `:$APP_LOCAL_PORT` (default `5176`), proxy di `/api` verso `$API_URL` (default `http://localhost:3030`) |
| `npm run build`   | build di produzione da zero (`scripts/build.mjs`); `-- --target <triple>` per un'altra piattaforma, `-- --xwin` per cargo-xwin |
| `npm run api-gen` | rigenera il client orval in `apps/web/src/api` dallo spec del server avviato |
| `npm run lint`    | eslint della pagina                                                  |
| `npm run format`  | prettier                                                             |

## Configurazione

`config.ini`, percorso da `--config` o variabile `QUICKSHARE_CONFIG` (default `./config.ini`). Il file è facoltativo: se manca valgono i default. `quickshare init` lo scrive dal template `apps/api/config.ini.template`.

| Chiave                        | Default              | Significato                                                     |
| ----------------------------- | -------------------- | --------------------------------------------------------------- |
| `Server.Bind` / `Server.Port` | `127.0.0.1` / `3030` | indirizzo di ascolto; l'IP dell'hotspot (es. `10.42.0.1`) per accettare connessioni dal telefono |
| `Storage.DataPath`            | `./data`             | cartella dati                                                   |
| `Upload.MaxBytes`             | `0`                  | dimensione massima di un file in byte; `0` = nessun limite      |
| `Log.Level`                   | `info`               | livello o direttive di filtro; `RUST_LOG`, se presente, ha la precedenza |

Il default di `Bind` è `127.0.0.1` perché esporre il server sulla rete è una scelta esplicita. Conviene l'IP dell'hotspot e non `0.0.0.0`: sul portatile ci sono altre interfacce (Docker, VPN) e solo con un IP preciso `scripts/serve.sh` sa in quale zona del firewall aprire la porta. Se l'hotspot non è attivo l'avvio fallisce con un messaggio esplicito.

## Sessione e token

Ogni avvio è una **sessione**: un token casuale (10 byte, esadecimale) generato all'avvio e mai salvato in config. Tutte le chiamate API tranne lo spec richiedono `Authorization: Bearer <token>`; la pagina lo legge dal parametro `t` dell'URL mostrato nel terminale, insieme a un QR code (stampato nero su bianco con colori ANSI, così si legge con qualunque tema del terminale). Senza token, o con un token di un avvio precedente, la pagina mostra "Link non valido". Il token serve a impedire che chiunque sia sull'hotspot carichi file; la password WPA resta la prima barriera.

## Hotspot

`scripts/hotspot.sh start [interfaccia] | stop | stato`, ripreso dal kit di estrazione: crea con NetworkManager una connessione `quickshare-hotspot` sulla prima interfaccia Wi-Fi (o su quella indicata), con SSID `QUICKSHARE` (o `QUICKSHARE_SSID`) e una password casuale di 12 caratteri senza simboli ambigui, e stampa SSID, password, IP da mettere in `Server.Bind` e, se c'è `qrencode`, il QR code per collegarsi al Wi-Fi. Avviandolo la scheda Wi-Fi lascia la rete attuale; `stop` elimina la connessione e NetworkManager torna alla rete abituale. NetworkManager assegna di solito lo stesso IP (`10.42.0.1`), quindi `Bind` in genere non cambia tra un avvio e l'altro.

Sequenza tipica: `scripts/hotspot.sh start`, il cliente inquadra il QR del Wi-Fi, `scripts/serve.sh`, il cliente inquadra il QR di quickshare, poi Ctrl+C e `scripts/hotspot.sh stop`.

## Firewall

Il binario non tocca il firewall. Lo fa `scripts/serve.sh [config.ini] [binario]`, che avvia quickshare dopo aver aperto la porta: se firewalld è attivo e `Server.Bind` è un IP non di loopback, trova l'interfaccia con quell'IP e la sua zona (`firewall-cmd --get-zone-of-interface`, o la zona di default) e apre la porta solo in quella zona e solo a runtime (`sudo firewall-cmd --zone=<zona> --add-port=<porta>/tcp`, senza `--permanent`), come faceva lo script SMB del kit di estrazione. Quando quickshare termina (Ctrl+C) la richiude con un `trap` su `EXIT`. Se la porta era già aperta non la tocca e non la chiude; con `Bind=0.0.0.0` non apre nulla e lo segnala; se nessuna interfaccia ha l'IP di `Bind` si ferma prima di avviare. Il binario va passato come secondo argomento, o con `QUICKSHARE_BIN`, altrimenti si cerca `quickshare` nel `PATH` e poi `./quickshare`. Una regola runtime sopravvissuta a un crash sparisce comunque al riavvio di firewalld.

## Layout su disco

```
data/                              # Storage.DataPath
  2026_10_13_17_01_22/             # un lotto (gruppo di upload): YYYY_MM_DD_HH_MM_SS in UTC
    quickshare.log                 # registro del lotto
    report.pdf
    report (1).pdf                 # stesso nome ricevuto due volte
  2026_10_13_17_01_22_2/           # secondo lotto iniziato nello stesso secondo
```

- Un **lotto** è un gruppo di file inviati con una pressione di Carica: la pagina crea il lotto (`POST /api/batches`), che apre la cartella e il registro, invia i file uno per uno, poi lo chiude. Il nome della cartella è l'ora di inizio in UTC, come tutti gli orari dei registri; l'id del lotto nell'API è il nome della cartella.
- I nomi dei file arrivano dal browser e vengono ripuliti (solo l'ultimo componente del percorso, niente caratteri di controllo o riservati su Windows, max 200 byte con l'estensione conservata). Un file non sovrascrive mai un altro: in caso di nome già presente si aggiunge ` (n)` prima dell'estensione, riservando il nome con `create_new`.

## Registro del lotto

`quickshare.log`, testo semplice. Gli orari sono in UTC seguiti dall'ora locale con offset, come nel registro dei casi del kit di estrazione:

```
QuickShare 0.1.0 (3f2a9c1d0b7e)
executable sha256: 9ae31d457817135390c6bce7ca824167d2dfff41d56ce5923493d28b0617695c
session started:   2026-10-13T17:01:12Z (2026-10-13 19:01:12 +02:00)
session token:     b2e92d97ff78fcba8396
batch started:     2026-10-13T17:01:22Z (2026-10-13 19:01:22 +02:00)
device:            smartphone · iPhone 17.5 · Safari 17.5
ip:                10.42.0.23
user agent:        Mozilla/5.0 (iPhone; ...)

[2026-10-13T17:01:30Z (2026-10-13 19:01:30 +02:00)] report (1).pdf
  original:   report.pdf
  size:       1.2 MB (1234567 bytes)
  sha256:     9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
  device:     smartphone · iPhone 17.5 · Safari 17.5
  ip:         10.42.0.23
  user agent: Mozilla/5.0 (iPhone; ...)

[2026-10-13T17:02:10Z (2026-10-13 19:02:10 +02:00)] upload not completed, partial file deleted: video.mp4

[2026-10-13T17:02:11Z (2026-10-13 19:02:11 +02:00)] batch closed: 1 files, 1.2 MB (1234567 bytes)
log sha256 (all previous lines): c3c51f0c89cb39e22de8f116cc262848afbe3492b33dfd373b59587f4791f17c
```

- L'intestazione riporta versione e commit del binario, l'hash dell'eseguibile in esecuzione, inizio e token della sessione, inizio del lotto e chi l'ha avviato.
- `original` compare solo se il file è stato rinominato. `device` è ricavato dallo user agent (crate `woothee`: categoria, sistema operativo, browser).
- Un invio interrotto (rete, schermo bloccato, file oltre il limite durante lo stream) lascia una riga `upload not completed` e il file parziale viene cancellato.
- Alla chiusura (dalla pagina a fine invio, oppure per tutti i lotti ancora aperti allo spegnimento) si scrive il riepilogo e, come ultima riga, lo SHA-256 di tutto ciò che precede, stampato anche nel terminale. Verifica: `head -n -1 quickshare.log | sha256sum`. Dopo la chiusura il lotto non accetta più file (`ERR_BATCH_NOT_FOUND`).

## Routing HTTP

| Percorso            | Contenuto                                                        |
| ------------------- | ---------------------------------------------------------------- |
| `/api/...`          | API con token — vedi [api.md](api.md)                            |
| `/api/openapi.json` | spec OpenAPI (senza token)                                       |
| `/`, `/{*path}`     | pagina (SPA): file inclusi nel binario, fallback su `index.html` |

I percorsi sono fissi: c'è una sola pagina e nessun motivo di montarla altrove.

## Terminale

Con `tracing` su stderr, orari in UTC (`2026-10-13T17:01:22Z`), senza target. All'avvio: versione con commit e hash dell'eseguibile, inizio e token della sessione, indirizzo di ascolto, poi per ogni indirizzo raggiungibile l'URL con il token e il suo QR code (con `0.0.0.0` uno per interfaccia IPv4, esclusi loopback e interfacce virtuali come `docker*`, `br-*`, `veth*`). Durante la sessione: `batch started` (cartella, dispositivo, IP), `file received` per ogni file con nome (e nome originale se rinominato), dimensione, sha256 e dispositivo, `upload not completed` per gli invii interrotti, `batch closed` con numero di file, dimensione e hash del registro. Le richieste HTTP sono loggate a livello `debug`.

## Spegnimento

Ctrl+C (o SIGTERM) smette di accettare connessioni e aspetta fino a 5 secondi gli upload in corso; quelli ancora aperti dopo vengono interrotti (i file parziali cancellati). Poi chiude tutti i lotti aperti, sigillandone i registri, e scrive `session closed`.

## CLI

```
quickshare [--config <path>]   # avvia il server
scripts/serve.sh [config] [bin] # lo avvia aprendo e poi richiudendo la porta su firewalld
quickshare --version           # versione e commit, es. 0.1.0 (3f2a9c1d0b7e)
quickshare init                # scrive un config.ini dal template (non sovrascrive)
quickshare openapi             # stampa lo spec OpenAPI su stdout
```

## Distribuzione

Un'installazione è il binario `quickshare` (pagina inclusa) ed eventualmente `config.ini`. `scripts/build.mjs` fa la build da zero come in Chemistry: cancella `apps/web/dist`, compila il server per la piattaforma della build, ne prende lo spec (`quickshare openapi`) per generare il client orval, compila la pagina, ricompila il crate per la piattaforma di destinazione così da includere la pagina aggiornata. Il binario è in `apps/api/target/release/` (o `target/<triple>/release/`).

**Versione e commit.** `build.rs` legge il commit con `git rev-parse --short=12 HEAD` e aggiunge `-dirty` se `git status --porcelain` non è vuoto (`unknown` fuori da un repository o prima del primo commit); la versione completa (`0.1.0 (3f2a9c1d0b7e)`) è in `--version`, nel terminale, nei registri e nel piè di pagina. Lo script di build ricompila sempre il crate, quindi il commit (e il `-dirty`) di un binario di produzione è sempre esatto; in sviluppo il build script si riesegue solo quando cambiano `.git/HEAD` o `.git/index`. Per un uso in tribunale: binario compilato da un commit pulito, hash dell'eseguibile nel verbale (lo stesso che compare nell'intestazione di ogni registro), sorgente e test disponibili.
