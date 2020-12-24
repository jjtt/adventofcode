#!/usr/bin/env python

import sys

neigh = [
    (1, -1, 0),
    (0, 1, -1),
    (-1, 1, 0),
    (0, -1, 1),
    (1, 0, -1),
    (-1, 0, 1),
]


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


def flip(blacks, x, y, z):
    num_blacks = sum(
        [blacks.get((x + x2, y + y2, z + z2), False) for x2, y2, z2 in neigh]
    )
    if blacks.get((x, y, z), False):
        return not (num_blacks == 0 or num_blacks > 2)
    else:
        return num_blacks == 2


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

for d in range(1, 101):
    print(f"Day: {d}")
    newblacks = {}
    keys = set(blacks.keys())
    for x, y, z in blacks.keys():
        keys.update(set([(x + x2, y + y2, z + z2) for x2, y2, z2 in neigh]))

    for x, y, z in keys:
        newblacks[(x, y, z)] = flip(blacks, x, y, z)

    blacks = newblacks

    print(sum(blacks.values()))
