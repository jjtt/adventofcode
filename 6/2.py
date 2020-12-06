#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

count = 0
one = set()
group = []
for l in lines + [""]:
    if l == "":
        for c in one:
            count = count + all([c in p for p in group])
        one = set()
        group = []
    else:
        one.update(list(l))
        group.append(set(list(l)))

print(count)
