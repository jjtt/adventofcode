#!/usr/bin/env python

import sys

def is_tree(lines, r, c):
    c = c % len(lines[r])
    return lines[r][c] == '#'


lines = [l.rstrip() for l in sys.stdin.readlines()]

trees = 0

c = 0

for r in range(len(lines)):
    if is_tree(lines, r, c):
        trees = trees + 1
    c = c + 3

print(trees)

