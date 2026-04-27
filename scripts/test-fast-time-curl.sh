#!/usr/bin/env -S bash

set -ueo pipefail


PORT="${PORT:-8080}"
HEALTH="http://localhost:${PORT}/health"
URL="http://localhost:${PORT}/"

HEADERS=(
    -H "Content-Type: application/json; charset=utf-8"
    -H "Accept: application/json, application/x-ndjson, text/event-stream"
)



INIT='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"demo","version":"0.0.1"}}}'

NOTIFY='{"jsonrpc": "2.0","method": "notifications/initialized"}'
LIST='{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
CALL='{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_system_time","arguments":{"timezone":"UTC"}}}'


echo $(curl -v -N $HEALTH)

curl -v -N "$URL" "${HEADERS[@]}" -d "$INIT" -D /tmp/headers.txt
# Mcp-Session-Id: mcp-session-7f1fdea1-49e9-40cf-b459-2ba0a57f8512
session=$(awk  -F: '($1=="Mcp-Session-Id"){print $2}' /tmp/headers.txt | tr -d ' \r')

printf "\n--- %s\n" $session

HEADERS=(
    -H "Mcp-Session-Id: ${session}"
    -H "Content-Type: application/json; charset=utf-8"
    -H "Accept: application/json, application/x-ndjson, text/event-stream"
)

curl  -N "$URL" "${HEADERS[@]}" -d "$NOTIFY"


printf "\n---\n"

curl  -N "$URL" "${HEADERS[@]}" -d "$LIST" | yq -P 
printf "\n---\n"
curl  -N "$URL" "${HEADERS[@]}" -d "$CALL" 
