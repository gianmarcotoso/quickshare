#!/usr/bin/env bash
# Avvia quickshare sull'IP attivo della macchina (l'hotspot, se acceso) aprendo la sua porta
# su firewalld, solo nella zona di quell'interfaccia e solo a runtime; la richiude all'uscita.
# Uso: scripts/serve.sh [--config <file>] [--bind <ip>] [--port <porta>]
#      le opzioni passano a quickshare; binario da QUICKSHARE_BIN, accanto allo script, dal PATH o ./quickshare
set -euo pipefail

HOTSPOT_CON=quickshare-hotspot

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -n "${QUICKSHARE_BIN:-}" ]]; then
  BIN="$QUICKSHARE_BIN"
elif [[ -x "$HERE/quickshare" ]]; then
  BIN="$HERE/quickshare"
else
  BIN="$(command -v quickshare || echo ./quickshare)"
fi
CONFIG="${QUICKSHARE_CONFIG:-config.ini}"
bind=""
port=""

args=("$@")
while (($#)); do
  case "$1" in
    --config) CONFIG="${2:-}"; shift ;;
    --config=*) CONFIG="${1#*=}" ;;
    --bind) bind="${2:-}"; shift ;;
    --bind=*) bind="${1#*=}" ;;
    --port) port="${2:-}"; shift ;;
    --port=*) port="${1#*=}" ;;
  esac
  shift
done

die() { echo "ERRORE: $*" >&2; exit 1; }

ini() {
  [[ -f "$CONFIG" ]] || return 0
  awk -F= -v section="$1" -v key="$2" '
    /^\[.*\]$/ { current = substr($0, 2, length($0) - 2); next }
    current == section && $1 == key { sub(/^[^=]*=/, ""); gsub(/^[ \t"]+|[ \t"]+$/, ""); print; exit }
  ' "$CONFIG"
}

hotspot_ip() {
  nmcli -g IP4.ADDRESS con show "$HOTSPOT_CON" 2>/dev/null | awk -F'[/ ]' 'NR == 1 && $1 != "" {print $1}' || true
}

default_route_ip() {
  ip -4 route get 1.1.1.1 2>/dev/null | awk '{for (i = 1; i < NF; i++) if ($i == "src") print $(i + 1)}' || true
}

[[ -x "$BIN" ]] || die "binario non trovato: $BIN"

if [[ -n "$bind" ]]; then
  echo "IP di ascolto: $bind (da --bind)"
elif bind="$(hotspot_ip)" && [[ -n "$bind" ]]; then
  echo "IP di ascolto: $bind (hotspot $HOTSPOT_CON)"
  args+=(--bind "$bind")
elif bind="$(default_route_ip)" && [[ -n "$bind" ]]; then
  echo "IP di ascolto: $bind (interfaccia della rotta di default; hotspot $HOTSPOT_CON non attivo)"
  args+=(--bind "$bind")
else
  die "nessun IP attivo trovato: avviare scripts/hotspot.sh start o passare --bind"
fi

[[ -n "$port" ]] || port="$(ini Server Port)"
port="${port:-3030}"
zone=""

close_port() {
  if [[ -n "$zone" ]]; then
    sudo firewall-cmd --zone="$zone" --remove-port="$port/tcp" >/dev/null &&
      echo "Porta $port/tcp chiusa nella zona $zone."
  fi
}

if [[ "$bind" == 127.* || "$bind" == "localhost" ]]; then
  :
elif ! systemctl is-active -q firewalld 2>/dev/null; then
  echo "firewalld non attivo: nessuna porta da aprire."
elif [[ "$bind" == "0.0.0.0" ]]; then
  echo "ATTENZIONE: ascolto su 0.0.0.0, porta non aperta: usare l'IP dell'hotspot (--bind)." >&2
else
  ifc="$(ip -4 -o addr show | awk -v ip="$bind" '!found && $4 ~ "^"ip"/" {print $2; found=1}')"
  [[ -n "$ifc" ]] || die "nessuna interfaccia con IP $bind: l'hotspot è attivo?"
  candidate="$(firewall-cmd --get-zone-of-interface="$ifc" 2>/dev/null || firewall-cmd --get-default-zone)"
  if firewall-cmd --zone="$candidate" --query-port="$port/tcp" >/dev/null 2>&1; then
    echo "Porta $port/tcp già aperta nella zona $candidate: non verrà chiusa."
  else
    sudo firewall-cmd --zone="$candidate" --add-port="$port/tcp" >/dev/null
    zone="$candidate"
    echo "Porta $port/tcp aperta nella zona $zone ($ifc), solo runtime."
  fi
fi

trap close_port EXIT
trap ':' INT TERM

status=0
"$BIN" "${args[@]}" || status=$?
exit "$status"
