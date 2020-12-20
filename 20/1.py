#!/usr/bin/env python

import sys

def combine_with_reversed(t):
    r = []
    for i in range(len(t)):
        r.append(chr(ord(t[i]) + ord(t[-i])))
    return r

def find_edges(t):
    e = [
        t[0],
        t[-1],
        [r[0] for r in t],
        [r[-1] for r in t],
    ]
    return [combine_with_reversed(t) for t in e]

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

print(edges)
print({e:v for e,v in edges.items() if len(v) < 2}.values())
l = [v[0] for e,v in edges.items() if len(v) < 2]
print(l)
l.sort()
print(l)
print(set(l))

