#!/usr/bin/env python
import sys

f = "input.txt"

if len(sys.argv) > 1:
    f = sys.argv[1]

instructions = open(f).readlines()[0].strip()

pos = (0, 0)

visits = {pos: 1}

for c in instructions:
    pos = {
        "<": (pos[0] - 1, pos[1]),
        ">": (pos[0] + 1, pos[1]),
        "^": (pos[0], pos[1] - 1),
        "v": (pos[0], pos[1] + 1),
    }[c]

    visits[pos] = visits.get(pos, 0) + 1

print(len([v for v in visits.values() if v > 0]))
