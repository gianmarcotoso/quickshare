#!/usr/bin/env bash
set -euo pipefail

if [[ $# -eq 0 ]]; then
	echo "uso: publish-gitea-release.sh <file-o-cartella>..." >&2
	exit 1
fi

release_id="$(curl -fsS -H "Authorization: token $GITEA_TOKEN" \
	"$API/releases/tags/$TAG" | jq -r '.id // empty' || true)"

if [ -z "$release_id" ]; then
	release_id="$(curl -fsS -X POST "$API/releases" \
		-H "Authorization: token $GITEA_TOKEN" -H "Content-Type: application/json" \
		-d "{\"tag_name\":\"$TAG\",\"name\":\"$TAG\"}" | jq -r '.id // empty')"
fi

if [ -z "$release_id" ]; then
	release_id="$(curl -fsS -H "Authorization: token $GITEA_TOKEN" \
		"$API/releases/tags/$TAG" | jq -r '.id')"
fi

find "$@" -type f | while IFS= read -r f; do
	name="$(basename "$f")"
	existing_id="$(curl -fsS -H "Authorization: token $GITEA_TOKEN" \
		"$API/releases/$release_id/assets" | jq -r --arg n "$name" '.[] | select(.name==$n) | .id' | head -n1)"
	if [ -n "$existing_id" ]; then
		curl -fsS -X DELETE -H "Authorization: token $GITEA_TOKEN" \
			"$API/releases/$release_id/assets/$existing_id" || true
	fi
	curl -fsS -X POST "$API/releases/$release_id/assets?name=$name" \
		-H "Authorization: token $GITEA_TOKEN" -F "attachment=@$f"
done
