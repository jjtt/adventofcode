#!/usr/bin/env python

lines = open("input.txt").readlines()

total = 0

for line in lines:
    dims = sorted(map(int, line.strip().split("x")))

    total += 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2]

print(total)
