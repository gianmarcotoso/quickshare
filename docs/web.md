# apps/web

Pagina di invio: React 19 + TypeScript, Vite (con React Compiler), Tailwind 4, componenti shadcn (stile `new-york`), TanStack Query, axios, client orval, Redux Toolkit (lingua), i18next (it/en), react-hot-toast, hash-wasm. Componenti e struttura del tema sono quelli di Chemistry (`index.css`, `components/ui`, `ThemeSwitch`, `LanguageSwitch`, `ActionButton`, `WithTooltip`); non ci sono router, form né tabelle perché c'è una sola pagina.

**Identità.** I colori vengono dall'icona: primario blu `#2f6fe0` (in tema scuro `#6c9cff`), inchiostro `#16181d`, fondo scuro `#14161a`, neutri freddi; successo, avviso e info restano quelli di Chemistry. Il logotipo nell'header è il testo `quickshare` in Space Grotesk 600 con `tracking-tight` (`@fontsource/space-grotesk`, `font-display`) accanto a `assets/quickshare-icon.svg`: i `logo-*.svg` non si usano nella pagina perché un SVG caricato come immagine non vede i font della pagina. `public/` contiene le icone del pacchetto (`favicon.svg` con tratti ispessiti per 16/32 px, PNG 16/32/48, `apple-touch-icon.png`, icone PWA normali e maskable) e `site.webmanifest`, tutti con percorsi relativi. Gli SVG originali (con i metadati C2PA) sono anche in `docs/assets/`, usati dal README.

## Struttura

```
src/
  main.tsx, index.css
  app/
    root.tsx                         # provider: QueryClient, tema, Redux, toaster
    features/uploads/
      uploads.page.tsx               # controllo del token, zona di scelta, selezione, lotti
      components/                    # file-drop-zone, selection-panel, batch-panel, file-card,
                                     # upload-status-badge, invalid-link
      hooks/use-uploads.hook.ts      # selezione, coda degli hash, invio dei lotti
      utils/                         # compute-sha256, tracked-file
  api/                               # GENERATO da orval — non modificare a mano
  assets/quickshare-icon.svg
  components/                        # header, progress-bar, app-version, switch di tema e lingua; ui/ = shadcn
  contexts/                          # theme-context.ts (tipi e contesto), theme-provider.tsx
  hooks/use-theme.hook.ts
  core/                              # http.ts (axios con il token, mutator orval), app-config.ts (versione, token da ?t=), i18n.ts
  store/                             # Redux Toolkit: slice language
  lib/                               # format, errori, utils, tema di sistema e applicazione al documento
  resources/locales/{it,en}.json
```

## Token

`AppConfig.Token` è il parametro `t` dell'URL; l'istanza axios lo manda come `Authorization: Bearer`. All'apertura la pagina chiama `GET /session` (`useCheckSession`): finché risponde mostra uno spinner, se fallisce mostra "Link non valido" al posto della pagina.

## Selezione e invio

I file si scelgono dal pulsante o trascinandoli sulla zona e finiscono tra i **file selezionati**, che restano modificabili: si possono aggiungere altri file, toglierne uno (X) o svuotare la selezione. Nulla parte finché non si preme **Carica**: allora la pagina crea un lotto sul server, sposta i file selezionati nel pannello del lotto ("Invio delle 19:01:22") e li invia uno alla volta con `uploadFile` di orval, passando `onUploadProgress` di axios per la barra di ogni file; alla fine chiude il lotto, anche se qualche invio è fallito. Uno alla volta perché sulla rete di un hotspot invii paralleli non vanno più veloci e rendono meno leggibile l'avanzamento. Mentre un lotto è in corso si può preparare la selezione successiva, ma Carica resta disattivato.

- **Hash**: lo SHA-256 parte appena un file è selezionato, in una coda sequenziale indipendente dall'invio, con hash-wasm e lettura a blocchi da 8 MiB (`File.slice`), così un file di qualche GB non viene caricato in memoria (Safari iOS chiuderebbe la pagina). Non si usa `crypto.subtle` perché esiste solo nei contesti sicuri (HTTPS o `localhost`), e la pagina aperta dal cliente è su `http://<ip>`; per lo stesso motivo gli id sono un contatore e non `crypto.randomUUID`. Un file rimosso prima che il suo hash parta esce dalla coda.
- **Verifica**: a invio completato l'hash restituito dal server viene confrontato con quello calcolato: badge "Inviato e verificato" se coincidono, "Impronta diversa" se no, "Inviato" se l'hash locale non è ancora pronto.
- **Errori**: un invio fallito mostra l'errore tradotto (`errors.<codice>`) e un pulsante Riprova che rimette il file (con l'hash già calcolato) tra i selezionati: sarà inviato in un nuovo lotto, perché quello di prima è chiuso.
- **Avviso**: durante un lotto il pannello mostra un avviso di tenere lo schermo acceso e la pagina aperta: se iOS blocca lo schermo o si cambia app, Safari interrompe l'upload e non c'è ripresa. Il Wake Lock del browser non è utilizzabile perché richiede HTTPS. Finché un lotto è in corso, chiudere la pagina chiede conferma (`beforeunload`).
- Alla fine di ogni lotto un toast riporta quanti file sono arrivati.

## Sviluppo

`npm run dev:api` e `npm run dev:web`; pagina su `http://localhost:5176/` (il dev server ascolta anche sulla rete, per provare da un telefono). Variabili da ambiente o `apps/web/.env.local` (modello in `.env.local.example`):

| Variabile        | Default                 | Uso                                                    |
| ---------------- | ----------------------- | ------------------------------------------------------ |
| `APP_LOCAL_PORT` | `5176`                  | porta del dev server                                   |
| `API_URL`        | `http://localhost:3030` | destinazione del proxy di `/api` e sorgente dello spec per orval |

`src/api/` non è versionato: dopo un clone va generato una prima volta con `npm run api-gen` a server avviato (oppure con `OPENAPI_SPEC=<file>` prodotto da `quickshare openapi`). Il build usa `base: './'`; la versione arriva dal meta iniettato dal server, in sviluppo dal plugin in `vite.config.ts` che la legge da `Cargo.toml` (senza commit). In sviluppo l'URL va aperto con il token: `http://localhost:5176/?t=<token mostrato dal server>`.
