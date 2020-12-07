#!/usr/bin/env python

import sys
import re

def normalize(s):
    s = s.replace('.', '')
    s = s.replace('bags', 'bag')
    return s

lines = [l.rstrip() for l in sys.stdin.readlines()]

for l in lines:
    outer, inner = re.split(r" contain ", l)
    outer = normalize(outer)
    inner = normalize(inner)
    inner = [s.strip() for s in inner.split(',')]
    print(outer, inner)
