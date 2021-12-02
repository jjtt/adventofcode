#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

def toid(s):
    return int(s.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1'), 2)

print(max([toid(s) for s in lines]))

