#!/usr/bin/env python

import sys

lines = [l.rstrip() for l in sys.stdin.readlines()]

count = 0
one = set()
for l in lines + [""]:
    if l == "":
        count = count + len(one)
        one = set()
    else:
        one.update(list(l))

print(count)
