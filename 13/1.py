#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

start = int(lines[0])

busses = []
for bus in lines[1].split(','):
    if bus != 'x':
        busses.append(int(bus))

for i in range(max(busses)):
    for bus in busses:
        if (start+i) % bus == 0:
            print(i * bus)
            exit()
