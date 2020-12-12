#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()] 

ints = [(i[:1], int(i[1:])) for i in lines]

d = 90
x = 0
y = 0

for (i,n) in ints:
    if i == 'R':
        d = (d + n) % 360
    if i == 'L':
        d = (d - n) % 360
    if i == 'F':
        if d == 0:
            y = y + n
        if d == 90:
            x = x + n
        if d == 180:
            y = y - n
        if d == 270:
            x = x - n
    if i == 'N':
        y = y + n
    if i == 'E':
        x = x + n
    if i == 'S':
        y = y - n
    if i == 'W':
        x = x - n

print(abs(x) + abs(y))
