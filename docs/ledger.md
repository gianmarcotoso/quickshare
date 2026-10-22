# Ledger

## 2026-10-13 — Primo impianto

- Progetto creato sulla base di Chemistry: stesso monorepo npm, crate Rust con Axum/utoipa/rust-embed, pagina React con shadcn, Tailwind, orval, Redux (lingua), i18next, tema e componenti condivisi. Tolto ciò che una pagina sola non usa (auth, database, router, informal, smart-table, date-fns).
- Una cartella di sessione per avvio (`data/YYYY_MM_DD_HH_MM_SS`) con `quickshare.log`, invece di una per invio: i file di una sessione col cliente restano insieme e il percorso è noto dall'avvio.
- `config.ini` facoltativo (non ci sono chiavi da generare); `Bind` di default `127.0.0.1`, l'esposizione su `0.0.0.0` è esplicita.
- Hash calcolato nel browser con hash-wasm perché `crypto.subtle` non è disponibile su `http://<ip>`; il server ricalcola l'hash in streaming e la pagina confronta i due.
- Ricezione in streaming su disco, nessun limite di dimensione di default, file parziali cancellati, nomi mai sovrascritti.

## 2026-10-13 — Lotti, token, registri sigillati

Dopo una revisione del primo impianto in vista dell'uso nel kit di estrazione:

- La cartella non è più per avvio ma per **lotto**: un gruppo di file inviati con una pressione di Carica. Prima ogni file partiva appena selezionato; ora la selezione è modificabile e l'invio parte solo da Carica. Il lotto si crea e si chiude esplicitamente (API `batches`); il Riprova di un file fallito lo rimette tra i selezionati invece di riaprire un lotto chiuso.
- Orari dei registri e del terminale in **UTC**, con l'ora locale e l'offset accanto nei registri (stessa convenzione del registro dei casi del kit); anche il nome della cartella è in UTC.
- Ogni registro di lotto si chiude con lo **SHA-256 delle righe precedenti** (`head -n -1 quickshare.log | sha256sum`), anche allo spegnimento: per questo lo spegnimento con Ctrl+C ora è ordinato.
- **Token di sessione** generato a ogni avvio, nell'URL e nel QR code mostrati nel terminale, richiesto da tutte le API; finisce nell'intestazione di ogni registro con l'ora di inizio della sessione.
- **Versione e commit** nel binario (`build.rs`) e **hash dell'eseguibile** calcolato all'avvio, entrambi nel terminale e nei registri, per poterli riportare nel verbale.
- **Apertura della porta** su firewalld, solo nella zona dell'interfaccia di `Bind` e solo a runtime, richiusa allo spegnimento, come faceva lo script SMB del kit (prima nel binario, poi spostata in uno script: vedi sotto). Il bind consigliato diventa l'IP dell'hotspot invece di `0.0.0.0`.
- **Avviso** sulla pagina durante l'invio: un upload interrotto da schermo bloccato non riprende. Upload a blocchi riprendibili scartati per ora: molto più lavoro per un caso che l'avviso copre.
- File parziali cancellati anche quando la richiesta viene abbandonata (guard al `drop`), con una riga nel registro.

## 2026-10-13 — Icone e identità

- Adottato il pacchetto icone "Impronta Q": favicon, icone PWA e manifest in `apps/web/public`, icona nell'header, loghi nel README. Il nome si scrive in minuscolo, `quickshare`.
- La palette del tema passa dal verde di Chemistry al blu dell'icona; il font del logotipo da Cormorant Garamond a Space Grotesk. La struttura del tema resta quella di Chemistry.

## 2026-10-13 — Firewall fuori dal binario

- L'apertura e la chiusura della porta su firewalld escono dal binario e passano a `scripts/serve.sh`, che avvia quickshare in mezzo. Il binario resta un server di file che non esegue `sudo` né modifica il sistema; la gestione del firewall, specifica della postazione, è uno script che si legge in un colpo d'occhio.
- Aggiunto `scripts/hotspot.sh`, ripreso dal kit di estrazione e reso autonomo (connessione `quickshare-hotspot`, SSID configurabile), così quickshare si usa anche senza il kit.
- `quickshare` accetta `--bind` e `--port` (precedenza sul config); `scripts/serve.sh` ricava da solo l'IP di ascolto (hotspot di quickshare se attivo, altrimenti la rete della rotta di default) e lo passa come `--bind`, così non serve scrivere l'IP nel config a ogni sessione.

## 2026-10-13 — Pipeline di release

- Ripresa da Chemistry la pipeline Gitea: immagini CI (`docker/ci-*.Dockerfile`, `build-images.yml`), release su tag per Linux musl, Windows (cargo-xwin) e macOS arm64, `package-release.sh` e `publish-gitea-release.sh`. Differenze: l'archivio Linux include `serve.sh` e `hotspot.sh` (solo Linux, perché dipendono da NetworkManager e firewalld), e ogni job dichiara il workspace `safe.directory` perché il commit nel binario sia corretto anche nei container.

## 2026-10-13 — Hotspot in WPA3

- Valutata la cifratura dei file nel browser (ChaCha20-Poly1305 con la chiave nel fragment del QR): chiuderebbe l'ascolto passivo da parte di chi conosce la password del Wi-Fi, ma non un attacco attivo, perché la pagina stessa arriva in HTTP e può essere sostituita. Scelto invece di portare l'hotspot a WPA3 (SAE), che chiude lo stesso caso senza codice nell'app; ripiego automatico su WPA2 e opzione `--wpa2` per i telefoni vecchi. La cifratura nell'app resta da riconsiderare solo se servisse davvero.
- `hotspot.sh` crea la connessione con `nmcli con add` invece di `nmcli dev wifi hotspot`, per poter scegliere sicurezza, PMF e cifrari.
- Il messaggio di errore all'avvio distingue porta già occupata e indirizzo non disponibile.

## 2026-10-22 — Licenza

- Progetto rilasciato sotto GPL-3.0 (`GPL-3.0-only`): testo in `LICENSE`, campo `license` in `Cargo.toml` e nei `package.json`.
