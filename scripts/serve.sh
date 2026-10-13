#!/usr/bin/env bash
# Avvia quickshare aprendo la sua porta su firewalld, solo nella zona dell'interfaccia
# di Server.Bind e solo a runtime; la richiude quando quickshare termina.
# Uso: scripts/serve.sh [config.ini] [percorso del binario]
set -euo pipefail

CONFIG="${1:-config.ini}"
BIN="${2:-${QUICKSHARE_BIN:-$(command -v quickshare || echo ./quickshare)}}"

die() { echo "ERRORE: $*" >&2; exit 1; }

ini() {
  [[ -f "$CONFIG" ]] || return 0
  awk -F= -v section="$1" -v key="$2" '
    /^\[.*\]$/ { current = substr($0, 2, length($0) - 2); next }
    current == section && $1 == key { sub(/^[^=]*=/, ""); gsub(/^[ \t"]+|[ \t"]+$/, ""); print; exit }
  ' "$CONFIG"
}

[[ -x "$BIN" ]] || die "binario non trovato: $BIN"

bind="$(ini Server Bind)"; bind="${bind:-127.0.0.1}"
port="$(ini Server Port)"; port="${port:-3030}"
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
  echo "ATTENZIONE: Server.Bind=0.0.0.0, porta non aperta: impostare l'IP dell'hotspot." >&2
else
  ifc="$(ip -4 -o addr show | awk -v ip="$bind" '$4 ~ "^"ip"/" {print $2; exit}')"
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
"$BIN" --config "$CONFIG" || status=$?
exit "$status"
