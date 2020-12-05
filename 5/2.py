#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

def toid(s):
    return int(s.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1'), 2)

seats = [toid(s) for s in lines]
seats.sort()

prev = seats[1]
for s in seats:
    if not s == prev+1:
        print(s)
    prev = s

