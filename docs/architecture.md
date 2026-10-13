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

`config.ini`, percorso da `--config` o variabile `QUICKSHARE_CONFIG` (default `./config.ini`). Il file è facoltativo: se manca valgono i default. `--bind` e `--port` hanno la precedenza su `Server.Bind` e `Server.Port`, così si può avviare sull'IP dell'hotspot senza toccare il config. `quickshare init` lo scrive dal template `apps/api/config.ini.template`.

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

`scripts/hotspot.sh start [interfaccia] [--wpa2] | stop | stato`, ripreso dal kit di estrazione: crea con NetworkManager una connessione `quickshare-hotspot` in modalità access point (2,4 GHz, IPv4 condiviso, IPv6 disattivato) sulla prima interfaccia Wi-Fi o su quella indicata, con SSID `QUICKSHARE` (o `QUICKSHARE_SSID`) e una password di due parole italiane separate da un trattino (es. `barca-riccio`, scelte a caso tra 115 con `/dev/urandom`), facile da dettare. Stampa SSID, password, sicurezza, IP, il comando di avvio (`scripts/serve.sh`, che userà da solo l'IP dell'hotspot) e, se c'è `qrencode`, il QR code per collegarsi al Wi-Fi. Serve `dnsmasq` installato (NetworkManager lo usa per il DHCP della connessione condivisa; senza, l'attivazione fallisce con "IP configuration could not be reserved"): lo script lo controlla prima di partire. Non serve `sudo`: NetworkManager autorizza l'utente della sessione. Avviandolo la scheda Wi-Fi lascia la rete attuale; `stop` elimina la connessione e NetworkManager torna alla rete abituale. NetworkManager assegna di solito lo stesso IP (`10.42.0.1`).

**Sicurezza del Wi-Fi.** Di default l'hotspot è **WPA3-Personal (SAE)** con PMF obbligatorio, solo RSN/CCMP. Con WPA2-PSK chi conosce la password e cattura l'associazione di un telefono può decifrarne tutto il traffico, file e token compresi; con SAE ogni sessione ha chiavi proprie e la password non basta. È la protezione del traffico che la pagina, servita in HTTP, non può dare da sola. Se la scheda non riesce ad attivare SAE in modalità access point lo script ripiega su WPA2 e lo segnala; `--wpa2` lo forza per i telefoni senza WPA3 (indicativamente prima di iOS 13 e Android 10). Il QR del Wi-Fi in WPA3 è `WIFI:T:WPA;R:1;…`: `R:1` è il "transition disable" del formato Wi-Fi Alliance e indica ai telefoni che lo capiscono di usare SAE; gli altri lo ignorano.

Sequenza tipica: `scripts/hotspot.sh start`, il cliente inquadra il QR del Wi-Fi, `scripts/serve.sh`, il cliente inquadra il QR di quickshare, poi Ctrl+C e `scripts/hotspot.sh stop`.

## Firewall

Il binario non tocca il firewall. Lo fa `scripts/serve.sh [--config <file>] [--bind <ip>] [--port <porta>]`, che avvia quickshare passandogli le stesse opzioni.

- **IP di ascolto.** Se non c'è `--bind`, lo script lo trova da solo e lo passa a quickshare come `--bind`: prima l'IP della connessione `quickshare-hotspot` se è attiva (`nmcli -g IP4.ADDRESS`), altrimenti l'IP sorgente della rotta di default (`ip route get 1.1.1.1`), cioè la rete a cui la macchina è collegata. Stampa quale ha scelto e perché; se non trova nulla si ferma. `Server.Bind` del config viene quindi ignorato quando si usa lo script; la porta viene da `--port`, poi da `Server.Port`, poi `3030`.
- **Porta.** Se firewalld è attivo e l'IP non è di loopback, trova l'interfaccia con quell'IP e la sua zona (`firewall-cmd --get-zone-of-interface`, o la zona di default) e apre la porta solo in quella zona e solo a runtime (`sudo firewall-cmd --zone=<zona> --add-port=<porta>/tcp`, senza `--permanent`), come faceva lo script SMB del kit di estrazione. Quando quickshare termina (Ctrl+C) la richiude con un `trap` su `EXIT`. Se la porta era già aperta non la tocca e non la chiude; con `--bind 0.0.0.0` non apre nulla e lo segnala.
- Il binario si indica con `QUICKSHARE_BIN`, altrimenti si cerca `quickshare` nel `PATH` e poi `./quickshare`.

Una regola runtime sopravvissuta a un crash sparisce comunque al riavvio di firewalld.

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
quickshare [--config <path>] [--bind <ip>] [--port <porta>]   # avvia il server
scripts/serve.sh [stesse opzioni]   # lo avvia aprendo e poi richiudendo la porta su firewalld
quickshare --version           # versione e commit, es. 0.1.0 (3f2a9c1d0b7e)
quickshare init                # scrive un config.ini dal template (non sovrascrive)
quickshare openapi             # stampa lo spec OpenAPI su stdout
```

## Distribuzione

Un'installazione è il binario `quickshare` (pagina inclusa) ed eventualmente `config.ini`. `scripts/build.mjs` fa la build da zero come in Chemistry: cancella `apps/web/dist`, compila il server per la piattaforma della build, ne prende lo spec (`quickshare openapi`) per generare il client orval, compila la pagina, ricompila il crate per la piattaforma di destinazione così da includere la pagina aggiornata. Il binario è in `apps/api/target/release/` (o `target/<triple>/release/`).

**Release** (`.gitea/workflows/release.yml`, ripresa da Chemistry): al push di un tag `v<versione>`, che deve coincidere con `version` di `apps/api/Cargo.toml` (unica fonte della versione), tre job compilano in parallelo e allegano l'archivio alla Release Gitea del tag (`scripts/publish-gitea-release.sh`, idempotente: un asset con lo stesso nome viene sostituito):

| Job | Dove | Target | Archivio |
| --- | --- | --- | --- |
| `build-linux` | container `quickshare-ci-linux` | `x86_64-unknown-linux-musl` (statico) | `quickshare-<versione>-linux-x86_64.tar.gz`, con `serve.sh` e `hotspot.sh` |
| `build-windows` | container `quickshare-ci-windows` | `x86_64-pc-windows-msvc` con cargo-xwin | `quickshare-<versione>-windows-x86_64.zip` |
| `build-macos` | runner self-hosted `macos-arm64`, senza container | nativo arm64 | `quickshare-<versione>-macos-arm64.tar.gz` |

`scripts/package-release.sh <binario> <piattaforma>` crea l'archivio in `build/<piattaforma>/`. Gli script stanno solo nell'archivio Linux, accanto al binario, perché usano NetworkManager, firewalld e `ip`; `serve.sh` cerca il binario prima nella propria cartella. Ogni job fa `git config --global --add safe.directory` prima della build: nei container il checkout ha un proprietario diverso dall'utente, git rifiuterebbe il repository e `build.rs` scriverebbe `unknown` al posto del commit. Le immagini dei container (`docker/ci-*.Dockerfile`: Ubuntu, Node, Rust stable con i target) sono costruite e pubblicate nel registry di Gitea da `.gitea/workflows/build-images.yml` quando cambiano i Dockerfile, con il secret `REGISTRY_TOKEN`.

**Build in locale con l'immagine CI**, per provare l'archivio Linux senza passare da Gitea (`.tmp/` è ignorata da git):

```sh
docker build -t quickshare-ci-linux -f docker/ci-linux-build.Dockerfile docker
rsync -a --delete --exclude node_modules --exclude target --exclude dist --exclude apps/web/src/api \
  --exclude .tmp --exclude build --exclude data --exclude config.ini ./ .tmp/src/
docker run --rm -v "$PWD/.tmp/src:/work" -w /work quickshare-ci-linux bash -c '
  git config --global --add safe.directory /work && npm ci &&
  npm run build -- --target x86_64-unknown-linux-musl &&
  bash scripts/package-release.sh apps/api/target/x86_64-unknown-linux-musl/release/quickshare linux-x86_64 &&
  chown -R '"$(id -u):$(id -g)"' /work'
tar -xzf .tmp/src/build/linux-x86_64/quickshare-*-linux-x86_64.tar.gz -C .tmp
```

La copia in `.tmp/src` evita che il container, che lavora come root, lasci file di root in `node_modules` e `target` del repository.

**Versione e commit.** `build.rs` legge il commit con `git rev-parse --short=12 HEAD` e aggiunge `-dirty` se `git status --porcelain` non è vuoto (`unknown` fuori da un repository o prima del primo commit); la versione completa (`0.1.0 (3f2a9c1d0b7e)`) è in `--version`, nel terminale, nei registri e nel piè di pagina. Lo script di build ricompila sempre il crate, quindi il commit (e il `-dirty`) di un binario di produzione è sempre esatto; in sviluppo il build script si riesegue solo quando cambiano `.git/HEAD` o `.git/index`. Per un uso in tribunale: binario compilato da un commit pulito, hash dell'eseguibile nel verbale (lo stesso che compare nell'intestazione di ogni registro), sorgente e test disponibili.
