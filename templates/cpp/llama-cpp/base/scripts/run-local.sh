#!/usr/bin/env bash
set -euo pipefail

if [ -f .env ]; then
  set -a
  . ./.env
  set +a
fi

MODEL_PATH="${MODEL_PATH:-./models/model.gguf}"
CTX_SIZE="${CTX_SIZE:-4096}"
THREADS="${THREADS:-8}"

./vendor/llama.cpp/build/bin/llama-cli \
  -m "$MODEL_PATH" \
  -c "$CTX_SIZE" \
  -t "$THREADS" \
  -f prompts/default.txt
