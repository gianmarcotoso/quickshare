<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/logo-dark.svg">
    <img src="docs/assets/logo-light.svg" alt="quickshare" width="320">
  </picture>
</p>

<p align="center">
  <strong>Trasferimento di file da telefono a computer, verificabile, senza installare nulla sul telefono.</strong>
</p>

<p align="center">
  <img alt="Versione" src="https://img.shields.io/badge/versione-0.1.1-2f6fe0">
  <img alt="Rust 2024" src="https://img.shields.io/badge/Rust-2024-b7410e?logo=rust&logoColor=white">
  <img alt="Axum 0.8" src="https://img.shields.io/badge/Axum-0.8-16181d">
  <img alt="React 19" src="https://img.shields.io/badge/React-19-61dafb?logo=react&logoColor=black">
  <img alt="TypeScript 6" src="https://img.shields.io/badge/TypeScript-6-3178c6?logo=typescript&logoColor=white">
  <img alt="Vite 8" src="https://img.shields.io/badge/Vite-8-646cff?logo=vite&logoColor=white">
  <img alt="Tailwind CSS 4" src="https://img.shields.io/badge/Tailwind_CSS-4-06b6d4?logo=tailwindcss&logoColor=white">
  <br>
  <img alt="SHA-256 end-to-end" src="https://img.shields.io/badge/SHA--256-end--to--end-2f6fe0">
  <img alt="Binario singolo" src="https://img.shields.io/badge/binario-singolo-6c9cff">
  <img alt="Piattaforme" src="https://img.shields.io/badge/piattaforme-Linux%20%7C%20macOS%20%7C%20Windows-5b6270">
</p>

---

Il computer crea un hotspot e avvia `quickshare`. Il cliente si collega, inquadra il QR code mostrato nel terminale, sceglie i file dal browser e preme **Carica**. Ogni file arriva in una cartella dedicata, con l'impronta SHA-256 calcolata **sia sul telefono sia sul computer** e un registro sigillato da riportare nel verbale.

Funziona con qualunque browser, iOS o Android: niente app, SMB, cavi o Debug USB.

## Caratteristiche

- **Verifica dell'integrità da un capo all'altro.** Il telefono calcola lo SHA-256 di ogni file prima dell'invio; il server lo ricalcola mentre il file arriva. La pagina li confronta e mostra *Inviato e verificato* (o *Impronta diversa*); il terminale stampa lo stesso hash.
- **Pensato anche per file grandi.** Il browser legge i file a blocchi e il server li scrive su disco man mano: un archivio di qualche GB non passa mai interamente dalla memoria, né sul telefono né sul computer.
- **Una cartella per ogni invio.** Ogni pressione di *Carica* crea `data/YYYY_MM_DD_HH_MM_SS/` (UTC) con i file e il loro `quickshare.log`.
- **Registro sigillato.** Per ogni file: nome, nome originale se rinominato, dimensione, SHA-256, dispositivo, IP e user agent, con orari in UTC e ora locale. Alla chiusura il registro termina con lo SHA-256 di tutte le righe precedenti.
- **Tracciabile.** Versione e commit sono compilati nel binario; l'hash dell'eseguibile in esecuzione compare nel terminale e in ogni registro.
- **Accesso solo con il link.** Ogni avvio genera un token di sessione, contenuto nell'URL e nel QR code: chi è sull'hotspot ma non ha il link non può caricare nulla.
- **Nessun file perso o alterato.** I nomi uguali vengono numerati (`report (1).pdf`) invece di sovrascrivere; gli invii interrotti vengono cancellati e annotati nel registro.
- **Wi-Fi in WPA3.** `scripts/hotspot.sh` crea un hotspot WPA3 (SAE): anche chi conosce la password non può decifrare il traffico degli altri. WPA2 resta disponibile per i telefoni che non supportano WPA3.
- **Firewall a parte.** `scripts/serve.sh` apre la porta su firewalld solo nella zona dell'hotspot e solo per la durata della sessione, poi la richiude; il binario non esegue `sudo` e non tocca il sistema.
- **Un solo binario.** La pagina web è inclusa nell'eseguibile; la configurazione è facoltativa.

## Uso

Requisiti per gli script: NetworkManager con `dnsmasq` (hotspot), `qrencode` facoltativo (QR del Wi-Fi), firewalld facoltativo.

```sh
scripts/hotspot.sh start               # hotspot WPA3: stampa SSID, password, IP e QR del Wi-Fi (--wpa2 per telefoni vecchi)
scripts/serve.sh                       # avvia sull'IP dell'hotspot (o della rete attiva), apre e poi richiude la porta
quickshare --bind 10.42.0.1            # oppure direttamente, senza toccare il firewall
```

```
IP di ascolto: 10.42.0.1 (hotspot quickshare-hotspot)
Porta 3030/tcp aperta nella zona nm-shared (wlp0s20f3), solo runtime.
2026-10-13T17:01:12Z  INFO quickshare version="0.1.1 (3f2a9c1d0b7e)" executable_sha256=9ae31d45…
2026-10-13T17:01:12Z  INFO session started started=2026-10-13T17:01:12Z (2026-10-13 19:01:12 +02:00) token=b2e92d97ff78fcba8396
2026-10-13T17:01:12Z  INFO open url=http://10.42.0.1:3030/?t=b2e92d97ff78fcba8396
  ▄▄▄▄▄▄▄ ▄▄  ▄ ▄▄▄▄▄▄▄
  █ ▄▄▄ █ ▀█▄█▀ █ ▄▄▄ █      (QR code)
  …
2026-10-13T17:01:30Z  INFO batch started folder=./data/2026_10_13_17_01_30 device=smartphone · iPhone 17.5 · Safari 17.5 ip=10.42.0.23
2026-10-13T17:01:41Z  INFO file received name=chat.zip size=1.2 GB sha256=6d6a6a1f… device=smartphone · iPhone 17.5 · Safari 17.5
2026-10-13T17:01:41Z  INFO batch closed folder=./data/2026_10_13_17_01_30 files=1 size=1.2 GB log_sha256=c3c51f0c…
```

**Ctrl+C** chiude la sessione: attende gli invii in corso e sigilla i registri ancora aperti; con `scripts/serve.sh` la porta viene poi richiusa.

A fine sessione `scripts/hotspot.sh stop` spegne l'hotspot e riconnette la rete abituale.

Per verificare un registro:

```sh
head -n -1 quickshare.log | sha256sum   # deve coincidere con l'ultima riga del file
```

### Configurazione

`--bind` e `--port` hanno la precedenza sul file; `quickshare init` crea un `config.ini` (facoltativo) per le altre impostazioni.

| Chiave                        | Default              | Significato                                       |
| ----------------------------- | -------------------- | ------------------------------------------------- |
| `Server.Bind` / `Server.Port` | `127.0.0.1` / `3030` | indirizzo di ascolto, di solito l'IP dell'hotspot |
| `Storage.DataPath`            | `./data`             | dove creare le cartelle degli invii               |
| `Upload.MaxBytes`             | `0`                  | dimensione massima di un file, `0` = nessun limite |
| `Log.Level`                   | `info`               | livello di log del terminale                      |

## Sul telefono

Una pagina web non compare nel menu *Condividi* delle app: il file va prima salvato sul dispositivo, poi scelto dalla pagina.

- **iOS:** *Esporta chat → Salva su File → Sul mio iPhone*, poi *Scegli i file* in Safari.
- **Android:** *Condividi → Salva* (o un file manager), poi *Scegli i file* nel browser.

Durante l'invio lo schermo deve restare acceso e la pagina aperta: se il telefono si blocca, Safari interrompe il caricamento, e il file va reinviato con *Riprova*.

## Sviluppo

Monorepo npm: `apps/api` (Rust, Axum) e `apps/web` (React, Vite, shadcn, Tailwind).

```sh
npm install
npm run dev:api        # server su :3030
npm run api-gen        # client orval dallo spec del server avviato
npm run dev:web        # pagina su :5176, aprire con ?t=<token>
npm run build          # binario di produzione con la pagina inclusa
```

Verifiche: `cargo test` e `cargo clippy --all-targets` in `apps/api`, `npx tsc -b` e `npm run lint` in `apps/web`.

La documentazione tecnica è in [`docs/`](docs/): [architettura](docs/architecture.md), [server](docs/api.md), [pagina](docs/web.md), [storia delle decisioni](docs/ledger.md).

## Licenza

GPL-3.0: vedi [`LICENSE`](LICENSE).
