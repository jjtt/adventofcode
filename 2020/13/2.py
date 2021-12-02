#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

busses = lines[-1].split(',')

high = max([int(b) for b in busses if b != 'x'])

e = {int(b):i for i,b in zip(range(len(busses)), busses) if b != 'x'}

i = 1
h = 18876031 * i + 3558031
x = h * 983 - 72


while any([(x + e[b]) % b for b in e.keys()]):
    if (i % 10000000) == 0:
        print((i,x))
    i = i + 1
    h = 18876031 * i + 3558031
    x = h * 983 - 72

print(x)
