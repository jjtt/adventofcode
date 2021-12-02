#!/usr/bin/env python

import sys

neighbours = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
]

def isval(seats, row, col, val):
    if row < 0 or col < 0 or row >= len(seats) or col >= len(seats[row]):
        return '.' in val
    return seats[row][col] in val

def empty(seats, row, col):
    global neighbours
    return all([isval(seats, row+r, col+c, 'L.') for r,c in neighbours]) 

def crowded(seats, row, col):
    global neighbours
    return 3 < sum([isval(seats, row+r, col+c, '#') for r,c in neighbours]) 

def seat(seats, row, col):
    cur = seats[row][col]
    if cur == 'L' and empty(seats, row, col):
        return '#'
    if cur == '#' and crowded(seats, row, col):
        return 'L'
    return seats[row][col]

lines = [l.rstrip() for l in sys.stdin.readlines()] 

while True:
    newlines = []
    for row in range(len(lines)):
        newrow = ''
        for col in range(len(lines[row])):
           newrow = newrow + seat(lines,row,col)
        newlines.append(newrow)
    if lines == newlines:
        break
    lines = newlines

for l in newlines:
    print(l)

print(sum([1 if item == '#' else 0 for sublist in newlines for item in sublist]))
