#!/usr/bin/env python
import sys

f = "input.txt"

if len(sys.argv) > 1:
    f = sys.argv[1]

instructions = open(f).readlines()[0].strip()

pos = [(0, 0), (0, 0)]

visits = {p: 1 for p in pos}

for i, c in enumerate(instructions):
    pos[i % len(pos)] = {
        "<": (pos[i % len(pos)][0] - 1, pos[i % len(pos)][1]),
        ">": (pos[i % len(pos)][0] + 1, pos[i % len(pos)][1]),
        "^": (pos[i % len(pos)][0], pos[i % len(pos)][1] - 1),
        "v": (pos[i % len(pos)][0], pos[i % len(pos)][1] + 1),
    }[c]

    visits[pos[i % len(pos)]] = visits.get(pos[i % len(pos)], 0) + 1

print(len([v for v in visits.values() if v > 0]))
