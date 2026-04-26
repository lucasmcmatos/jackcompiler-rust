#!/usr/bin/env bash

# Valida os XMLs gerados pelos analisadores lexico e sintatico
# contra os arquivos oficiais do pacote Square do nand2tetris.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="$ROOT_DIR/square-tests/output"
VALIDATION_DIR="$ROOT_DIR/square-validation"

validate_pair() {
    local generated="$1"
    local expected="$2"
    local label="$3"

    if [[ ! -f "$generated" ]]; then
        echo "ERRO: arquivo gerado nao encontrado: $generated"
        exit 1
    fi

    if [[ ! -f "$expected" ]]; then
        echo "ERRO: arquivo oficial nao encontrado: $expected"
        exit 1
    fi

    if diff -w "$generated" "$expected" > /dev/null; then
        echo "OK: $label"
    else
        echo "ERRO: diferenca encontrada em $label"
        diff -w "$generated" "$expected"
        exit 1
    fi
}

for class_name in Main Square SquareGame; do
    validate_pair \
        "$OUTPUT_DIR/${class_name}T.xml" \
        "$VALIDATION_DIR/${class_name}T.xml" \
        "${class_name} tokens"

    validate_pair \
        "$OUTPUT_DIR/${class_name}P.xml" \
        "$VALIDATION_DIR/${class_name}.xml" \
        "${class_name} parser"
done

echo "Validacao Square concluida com sucesso."
