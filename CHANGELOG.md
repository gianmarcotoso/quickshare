# Changelog

Cambiamenti visibili a chi usa QuickShare, dal più recente. Le versioni seguono il [versionamento semantico](https://semver.org/lang/it/).

## [0.1.1] — 2026-10-22

### Aggiunto

- Licenza GPL-3.0 (`LICENSE`).

### Modificato

- Ritocchi grafici nell'intestazione della pagina: spaziatura del logotipo e dimensione dell'icona del tema.

## [0.1.0] — 2026-10-13

Prima versione.

- Pagina web inclusa nel binario, da cui un dispositivo collegato alla stessa rete invia uno o più file.
- Accesso con un token di sessione generato a ogni avvio, contenuto nell'URL e nel QR code mostrati nel terminale.
- SHA-256 calcolato sia nel browser sia sul server e confrontato a ogni invio.
- Una cartella per ogni invio, con un registro in UTC sigillato dallo SHA-256 delle righe precedenti.
- Nomi duplicati numerati invece di sovrascritti; invii interrotti cancellati e annotati nel registro.
- Versione, commit e hash dell'eseguibile nel terminale e in ogni registro.
- Script `hotspot.sh` (hotspot WPA3) e `serve.sh` (apertura temporanea della porta su firewalld).
- Archivi per Linux x86_64, Windows x86_64 e macOS arm64.

[0.1.1]: https://github.com/gianmarcotoso/quickshare/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gianmarcotoso/quickshare/releases/tag/v0.1.0
