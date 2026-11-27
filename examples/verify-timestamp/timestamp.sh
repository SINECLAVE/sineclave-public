#!/usr/bin/env bash
# Get an RFC 3161 time-stamp for a file from the public Sineclave Trust Node and verify it.
# Needs only OpenSSL (1.1.1 or newer) and curl. Nothing from Sineclave is installed.
#
#   ./timestamp.sh contract.pdf          -> contract.pdf.tsr (the time-stamp token)
#
set -euo pipefail

TSA_URL="${TSA_URL:-https://trustnode.sineclave.com/tsa}"
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="${ROOT_PEM:-$HERE/sineclave-trust-root.pem}"

f="${1:?usage: $0 <file>}"
[ -f "$f" ] || { echo "no such file: $f" >&2; exit 1; }
out="$f.tsr"
req="$(mktemp)"; tmp="$(mktemp)"; trap 'rm -f "$req" "$tmp"' EXIT

# 1. The request carries only a SHA-384 hash of the file; the file itself never leaves your machine.
openssl ts -query -data "$f" -sha384 -cert -out "$req"

# 2. Send it to the time-stamp authority.
code=$(curl -sS -o "$tmp" -w '%{http_code}' -H 'Content-Type: application/timestamp-query' \
            --data-binary @"$req" "$TSA_URL")
if [ "$code" = 429 ]; then
  echo "the public demo allows 5 time-stamps per minute per address - try again in a minute" >&2; exit 2
fi
[ "$code" = 200 ] || { echo "time-stamp server answered HTTP $code" >&2; exit 1; }

mv "$tmp" "$out"

# 3. Show what was signed, then verify the token against the root certificate kept in this repository
#    (not against anything the server sends), and against the file.
openssl ts -reply -in "$out" -text | grep -E 'Status:|Hash Algorithm|Time stamp|Serial number|TSA:'
openssl ts -verify -data "$f" -in "$out" -CAfile "$ROOT"

echo
echo "Token saved as $out. Check it again at any time, offline:"
echo "  openssl ts -verify -data \"$f\" -in \"$out\" -CAfile \"$ROOT\""
