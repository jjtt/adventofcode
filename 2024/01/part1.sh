#!/usr/bin/env bash

awk '{total += $1 > $2 ? $1 - $2 : $2 - $1} END {print total}' <(paste <(cut -d' ' -f1 input.txt| sort) <(cut -d' ' -f4 input.txt| sort))
