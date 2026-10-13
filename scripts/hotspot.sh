#!/usr/bin/env bash
# Hotspot Wi-Fi dedicato a quickshare, con NetworkManager (ripreso dal kit di estrazione).
# Uso: scripts/hotspot.sh start [interfaccia] | stop | stato
# Nota: avviandolo, la scheda Wi-Fi si scollega dalla rete attuale.
set -euo pipefail

CON=quickshare-hotspot
SSID="${QUICKSHARE_SSID:-QUICKSHARE}"

WORDS=(
  acqua albero alba ancora anello aquila arancia argento autunno balena barca bosco bottone
  brezza bussola campo candela canto carta castello cavallo cielo ciliegia cipresso collina
  coniglio corda cortile cuscino delfino diamante duna edera estate falco farfalla faro
  fiume foglia fontana formica fragola fulmine gabbiano gatto gelato ghianda giardino girasole
  gomitolo grano grotta isola lago lampada lanterna lepre limone luna lupo mandorla mare
  margherita matita melone miele mirtillo montagna mulino neve nido nocciola nuvola oliva
  onda orso ortica pane panda passero pesca pietra pigna pino pioggia piuma ponte porto
  prato quercia radice rana riccio rondine rosa sabbia sasso scoglio seme sentiero sole
  stella tamburo tigre torre treno trifoglio tulipano uva valle vento vela viola volpe zucca
)

die() { echo "ERRORE: $*" >&2; exit 1; }

random_word() {
  local n
  n="$(od -An -N4 -tu4 /dev/urandom)"
  echo "${WORDS[$(( ${n// /} % ${#WORDS[@]} ))]}"
}

case "${1:-}" in
  start)
    command -v dnsmasq >/dev/null ||
      die "dnsmasq non installato: NetworkManager lo usa per assegnare gli IP ai dispositivi (sudo pacman -S dnsmasq)"
    ifc="${2:-$(nmcli -t -f DEVICE,TYPE dev | awk -F: '!found && $2=="wifi" {print $1; found=1}')}"
    [[ -n "$ifc" ]] || die "nessuna interfaccia Wi-Fi trovata"
    pw="$(random_word)-$(random_word)"
    nmcli con delete "$CON" >/dev/null 2>&1 || true
    nmcli dev wifi hotspot ifname "$ifc" con-name "$CON" ssid "$SSID" password "$pw" >/dev/null
    sleep 2
    ip="$(ip -4 -o addr show dev "$ifc" | awk '{print $4}' | cut -d/ -f1)"
    echo "Hotspot attivo su $ifc"
    echo "  SSID:     $SSID"
    echo "  Password: $pw"
    echo "  IP:       $ip"
    echo "Avvio:      scripts/serve.sh"
    if command -v qrencode >/dev/null; then
      qrencode -t ansiutf8 "WIFI:T:WPA;S:$SSID;P:$pw;;"
    fi
    ;;
  stop)
    nmcli con down "$CON" >/dev/null 2>&1 || true
    nmcli con delete "$CON" >/dev/null 2>&1 || true
    echo "Hotspot fermato (NetworkManager riconnette la rete abituale)."
    ;;
  stato)
    nmcli -f NAME,DEVICE,STATE con show --active | grep -E "^(NAME|$CON)" || echo "Hotspot non attivo"
    ;;
  *) die "uso: $0 start [interfaccia] | stop | stato" ;;
esac
