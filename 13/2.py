#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

busses = lines[-1].split(',')

high = max([int(b) for b in busses if b != 'x'])

e = {int(b):i for i,b in zip(range(len(busses)), busses) if b != 'x'}

i = 1
while any([((i*high) - e[high] + e[b]) % b for b in e.keys()]):
    if (i % 10000000) == 0:
        print(i)
    i = i + 1

print(i)
print((i*high) - e[high])
