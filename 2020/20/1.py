#!/usr/bin/env python

import sys
import math

def find_edges(t):
    e = [
        t[0],
        [r[-1] for r in t],
        list(reversed(t[-1])),
        list(reversed([r[0] for r in t])),
    ]
    return e + [list(reversed(t)) for t in e]

lines = [l.rstrip() for l in sys.stdin.readlines()] 

rules = {}

tiles = {}
tile = None
for l in lines:
    if l == "":
        continue
    elif l.startswith('Tile'):
        tile = int(l[5:-1])
        tiles[tile] = []
    else:
        tiles[tile].append(list(l))


edges = {}
for t in tiles.keys():
    e = find_edges(tiles[t])
    for edge in e:
        key = ''.join(edge)
        nums = edges.get(key, [])
        nums.append(t)
        edges[key] = nums

l = [v[0] for e,v in edges.items() if len(v) < 2]

print(math.prod(set([i for i in l if l.count(i) == 4])))


