#!/usr/bin/env bash

set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
    echo "This script requires the GitHub CLI." >&2
    echo "https://cli.github.com" >&2
    exit 1
fi

F="$(mktemp)"
trap 'rm -f "$F"' EXIT

printf 'votes,feature,"issue url"\n' > "$F"

gh issue list \
    --repo="actix/actix-web-lab" \
    --limit=999 \
    --search="is:issue is:open label:poll sort:reactions-+1-desc" \
    --json="title,url,reactionGroups" \
    --jq '
        .[]
        | [
            ((.reactionGroups[]? | select(.content == "THUMBS_UP") | .users.totalCount) // 0),
            (.title | sub("^\\[poll\\] "; "")),
            .url
        ]
        | @csv
        ' >> "$F"

qsv table < "$F"
