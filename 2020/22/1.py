#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()] 

players = [[],[]]
player = 0
for l in lines:
    if l.startswith('Player '):
        continue
    if l == '':
        player = 1
        continue
    players[player].append(int(l))

while 0 < min([len(p) for p in players]):
    current = [p.pop(0) for p in players]
    winner = current.index(max(current))
    current.sort()
    current.reverse()
    players[winner].extend(current)

winlist = players[winner]

print(sum([x*y for x,y in zip(reversed(winlist), range(1,len(winlist)+1))]))
