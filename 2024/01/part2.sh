#!/usr/bin/env bash

awk '
{
    left[NR] = $1;
    right[$2]++
}
END {
    for (i=1; i<=NR; i++) {
        sum+=left[i] * right[left[i]]
    }
    print sum
}' input.txt
