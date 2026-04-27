


URL=http://localhost:8080


mcp-inspector --cli "$URL" --transport http --method tools/list | yq -P

echo ---

mcp-inspector --cli "$URL" --transport http --method tools/call \
	--tool-name get_system_time \
	--tool-arg timezone=UTC | yq -P 
