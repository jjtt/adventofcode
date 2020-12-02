#!/bin/bash

year=${2:-2020}

grep -f <(awk "{print \"^\" ($year - \$1) \"$\"}" $1) $1 | paste - - | awk '{print $1 * $2}'
