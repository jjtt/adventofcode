#!/usr/bin/env python

import sys


def pick(s):
    if s.startswith("ne"):
        return ((1, -1, 0), s[2:])
    elif s.startswith("se"):
        return ((0, 1, -1), s[2:])
    elif s.startswith("sw"):
        return ((-1, 1, 0), s[2:])
    elif s.startswith("nw"):
        return ((0, -1, 1), s[2:])
    elif s.startswith("e"):
        return ((1, 0, -1), s[1:])
    elif s.startswith("w"):
        return ((-1, 0, 1), s[1:])


lines = [l.rstrip() for l in sys.stdin.readlines()]

blacks = {}

for l in lines:
    x, y, z = 0, 0, 0
    while l:
        (x2, y2, z2), l = pick(l)
        x += x2
        y += y2
        z += z2
    black = blacks.get((x, y, z), False)
    blacks[(x, y, z)] = not black

print(sum(blacks.values()))
