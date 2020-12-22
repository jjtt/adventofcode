#!/usr/bin/env python

import sys
import copy

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

def round(players,visited):
    if players in visited:
        return players,0

    visited.append(copy.deepcopy(players))

    current = [p.pop(0) for p in players]

    if all([current[i] <= len(players[i]) for i in range(len(players))]):
        newplayers = [players[i][:current[i]] for i in range(len(players))]
        _,winner = round(newplayers, [])
    else:
        winner = current.index(max(current))

    sortedpair = [current.pop(winner)]
    sortedpair.extend(current)
    players[winner].extend(sortedpair)

    if 0 < min([len(p) for p in players]):
        return round(players,visited)

    return players,winner

players,winner = round(players,[])
winlist = players[winner]

print(sum([x*y for x,y in zip(reversed(winlist), range(1,len(winlist)+1))]))
