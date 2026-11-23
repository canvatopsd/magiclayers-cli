#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
project=$(git rev-parse --show-toplevel)
artifact="$project/.tmp/build/cli/release/magiclayers"
test -x "$artifact"
file "$artifact" | rg 'Mach-O|ELF'
version=$(node deployment/release.mjs)
test "$("$artifact" --version)" = "magiclayers $version"
"$artifact" discover | node -e 'let s="";process.stdin.on("data",c=>s+=c).on("end",()=>{const v=JSON.parse(s);if(!v.ok||v.data.schemaVersion!==2)process.exit(1)})'
