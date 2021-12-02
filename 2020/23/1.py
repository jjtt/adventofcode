#!/usr/bin/env python

import sys

def pick(inputcircle, start, count):
    circle = inputcircle[:]
    start = start % len(circle)
    a = circle[start:start+count]
    del circle[start:start+count]
    if not len(a) == count:
        b = circle[:count-len(a)]
        del circle[:count-len(a)]
        a = a + b
    return a, circle

def dest(rest, current, size):
    while True:
        current = (current - 1) % size
        if current == 0:
            current = size
        if current in rest:
            return current

rounds = int(sys.argv[1])

lines = [l.rstrip() for l in sys.stdin.readlines()] 

clockwise = [int(i) for i in lines[0]]
current = clockwise[0]


for i in range(rounds):
    ci = clockwise.index(current)
    p, r = pick(clockwise, ci+1, 3)
    d = dest(r, clockwise[ci], len(clockwise))

    di = r.index(d)+1
    r[di:di] = p
    clockwise = r
    current = clockwise[(clockwise.index(current) + 1) % len(clockwise)]
    
one = clockwise.index(1)
out = []
for i in range(one+1, one+len(clockwise)):
    out.append(clockwise[i%len(clockwise)])
print(''.join([str(i) for i in out]))
