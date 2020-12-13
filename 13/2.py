#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

busses = lines[-1].split(',')

e = [f"(x+{i})%{b}" for i,b in zip(range(len(busses)), busses) if b != 'x'] + ['0']

print('='.join(e))
