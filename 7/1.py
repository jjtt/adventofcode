#!/usr/bin/env python

import sys
import re
import pprint

def normalize(s):
    s = s.replace('.', '')
    s = s.replace('bags', 'bag')
    return s

def num(s):
    i, n = s.split(' ', 1)
    try:
        return int(i), n
    except ValueError:
        return 0, n

lines = [l.rstrip() for l in sys.stdin.readlines()]

db = {}

for l in lines:
    outer, inner = re.split(r" contain ", l)
    outer = normalize(outer)
    inner = normalize(inner)
    inner = [num(s.strip()) for s in inner.split(',')]
    for n,i in inner:
        li = db.get(outer, [])
        if not i == 'other bag':
            li.append(i)
        db[outer] = li

pprint.pp(db)

