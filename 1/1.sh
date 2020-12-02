#!/bin/bash

tmp=$(mktemp)
cat <"$1" > "$tmp"

year=${2:-2020}

grep -f <(awk "{print \"^\" ($year - \$1) \"$\"}" $tmp) $tmp | paste - - | awk '{print $1 * $2}'

rm "$tmp"
