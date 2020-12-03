#!/usr/bin/env python

import sys

def is_tree(lines, r, c):
    c = c % len(lines[r])
    return lines[r][c] == '#'


lines = [l.rstrip() for l in sys.stdin.readlines()]

def slope(lines, down, right):
    trees = 0

    r = 0
    c = 0

    while r < len(lines):
        if is_tree(lines, r, c):
            trees = trees + 1
        c = c + right
        r = r + down
    return trees

print( slope(lines, 1, 1) * slope(lines, 1, 3) * slope(lines, 1, 5) * slope(lines, 1, 7) * slope(lines, 2, 1))

