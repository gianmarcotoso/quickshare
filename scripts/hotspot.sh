#!/usr/bin/env bash
# Hotspot Wi-Fi dedicato a quickshare, con NetworkManager (ripreso dal kit di estrazione).
# Uso: scripts/hotspot.sh start [interfaccia] [--wpa2] | stop | stato
# WPA3 (SAE) di default, con ripiego automatico su WPA2 se la scheda non lo supporta.
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

activate() {
  local key_mgmt="$1" pmf=default
  [[ "$key_mgmt" == sae ]] && pmf=required

  nmcli con delete "$CON" >/dev/null 2>&1 || true
  nmcli con add type wifi ifname "$ifc" con-name "$CON" autoconnect no ssid "$SSID" \
    802-11-wireless.mode ap 802-11-wireless.band bg \
    ipv4.method shared ipv6.method disabled \
    wifi-sec.key-mgmt "$key_mgmt" wifi-sec.psk "$pw" wifi-sec.pmf "$pmf" \
    wifi-sec.proto rsn wifi-sec.pairwise ccmp wifi-sec.group ccmp >/dev/null
  nmcli con up "$CON" >/dev/null 2>&1
}

case "${1:-}" in
  start)
    shift
    security=sae
    ifc=""
    for arg in "$@"; do
      case "$arg" in
        --wpa2) security=wpa-psk ;;
        -*) die "opzione sconosciuta: $arg" ;;
        *) ifc="$arg" ;;
      esac
    done
    command -v dnsmasq >/dev/null ||
      die "dnsmasq non installato: NetworkManager lo usa per assegnare gli IP ai dispositivi (sudo pacman -S dnsmasq)"
    [[ -n "$ifc" ]] || ifc="$(nmcli -t -f DEVICE,TYPE dev | awk -F: '!found && $2=="wifi" {print $1; found=1}')"
    [[ -n "$ifc" ]] || die "nessuna interfaccia Wi-Fi trovata"
    pw="$(random_word)-$(random_word)"
    if [[ "$security" == sae ]] && ! activate sae; then
      echo "ATTENZIONE: WPA3 non attivabile su $ifc, ripiego su WPA2." >&2
      security=wpa-psk
    fi
    if [[ "$security" == wpa-psk ]]; then
      activate wpa-psk || {
        nmcli con delete "$CON" >/dev/null 2>&1 || true
        die "attivazione dell'hotspot non riuscita (journalctl -u NetworkManager)"
      }
    fi
    sleep 2
    ip="$(ip -4 -o addr show dev "$ifc" | awk '{print $4}' | cut -d/ -f1)"
    echo "Hotspot attivo su $ifc"
    echo "  SSID:      $SSID"
    echo "  Password:  $pw"
    if [[ "$security" == sae ]]; then
      echo "  Sicurezza: WPA3 (SAE)   per telefoni senza WPA3: $0 start --wpa2"
      wifi_qr="WIFI:T:WPA;R:1;S:$SSID;P:$pw;;"
    else
      echo "  Sicurezza: WPA2"
      wifi_qr="WIFI:T:WPA;S:$SSID;P:$pw;;"
    fi
    echo "  IP:        $ip"
    echo "Avvio:       scripts/serve.sh"
    if command -v qrencode >/dev/null; then
      qrencode -t ansiutf8 "$wifi_qr"
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
  *) die "uso: $0 start [interfaccia] [--wpa2] | stop | stato" ;;
esac
