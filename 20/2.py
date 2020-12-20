#!/usr/bin/env python

import sys
import math

def printtile(t):
    for r in t:
        print(''.join(r))

def find_edges(t):
    e = [
        t[0],
        [r[-1] for r in t],
        list(reversed(t[-1])),
        list(reversed([r[0] for r in t])),
    ]
    return e + [list(reversed(t)) for t in e]

def flip(t):
    return [list(reversed(r)) for r in t]

def turn(t):
    trans = []
    for c in range(len(t)):
        row = []
        for r in range(len(t)):
            row.append(t[r][c])
        trans.append(row)
    return(flip(trans))

def turn_to_bottom(t, bottom):
    pass
    

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

first = [i for i in l if l.count(i) == 4][0]
firstedges = list(set([''.join(e) for e in find_edges(tiles[first]) if len(edges[''.join(e)]) > 1]))
firstbottom = firstedges[0]
firstright = [e for e in firstedges[1:] if not e == firstbottom[::-1]][0]

printtile(tiles[first])
print()
printtile(flip(tiles[first]))
print()
printtile(turn(flip(tiles[first])))

print(first)
print(firstedges)
print(firstbottom)
print(firstright)

fixed = [first]

neigh = [edges[''.join(e)] for e in find_edges(tiles[first])]

print(neigh)

print([item for sublist in neigh for item in sublist if item not in fixed][0])





