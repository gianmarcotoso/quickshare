#!/usr/bin/env bash
# Hotspot Wi-Fi dedicato a quickshare, con NetworkManager (ripreso dal kit di estrazione).
# Uso: scripts/hotspot.sh start [interfaccia] | stop | stato
# Nota: avviandolo, la scheda Wi-Fi si scollega dalla rete attuale.
set -euo pipefail

CON=quickshare-hotspot
SSID="${QUICKSHARE_SSID:-QUICKSHARE}"

die() { echo "ERRORE: $*" >&2; exit 1; }

case "${1:-}" in
  start)
    ifc="${2:-$(nmcli -t -f DEVICE,TYPE dev | awk -F: '$2=="wifi"{print $1; exit}')}"
    [[ -n "$ifc" ]] || die "nessuna interfaccia Wi-Fi trovata"
    pw="$(tr -dc 'a-km-z2-9' </dev/urandom | head -c 12)"
    nmcli con delete "$CON" >/dev/null 2>&1 || true
    nmcli dev wifi hotspot ifname "$ifc" con-name "$CON" ssid "$SSID" password "$pw" >/dev/null
    sleep 2
    ip="$(ip -4 -o addr show dev "$ifc" | awk '{print $4}' | cut -d/ -f1)"
    echo "Hotspot attivo su $ifc"
    echo "  SSID:     $SSID"
    echo "  Password: $pw"
    echo "  IP:       $ip   (in config.ini: [Server] Bind=$ip)"
    command -v qrencode >/dev/null && qrencode -t ansiutf8 "WIFI:T:WPA;S:$SSID;P:$pw;;"
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
