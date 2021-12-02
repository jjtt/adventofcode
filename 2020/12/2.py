#!/usr/bin/env python

import sys
import math

lines = [l.rstrip() for l in sys.stdin.readlines()] 

ints = [(i[:1], int(i[1:])) for i in lines]

x = 0
y = 0
wx = 10
wy = 1

for (i,n) in ints:
    print(i,n)
    if i == 'R':
        i = 'L'
        n = -n
    if i == 'L':
        r = math.radians(n)
        tx = wx
        ty = wy
        wx = int(int(tx * math.cos(r)) - int(ty * math.sin(r)))
        wy = int(int(ty * math.cos(r)) + int(tx * math.sin(r)))
    if i == 'F':
        y = y + n * wy
        x = x + n * wx
    if i == 'N':
        wy = wy + n
    if i == 'E':
        wx = wx + n
    if i == 'S':
        wy = wy - n
    if i == 'W':
        wx = wx - n
    print((x,y),(wx,wy))

print(abs(x) + abs(y))
