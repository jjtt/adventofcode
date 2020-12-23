#!/usr/bin/env python

import sys
from collections import deque

def pick(circle, start, count):
    #circle = inputcircle[:]
    start = start % len(circle)
    end = start+count
    a = circle[start:end]
    #del circle[start:start+count]
    if not len(a) == count:
        end = count-len(a)
        b = circle[:end]
        #del circle[:count-len(a)]
        a = a + b
    return a, end

def dest(picked, current, size):
    while True:
        current = (current - 1) % size
        if current == 0:
            current = size
        if current not in picked:
            return current

rounds = int(sys.argv[1])

lines = [l.rstrip() for l in sys.stdin.readlines()] 

clockwise = [int(i) for i in lines[0]]

size = int(sys.argv[2])
cupcount = size - len(clockwise)

extracups = range(max(clockwise)+1, max(clockwise)+cupcount+1)

clockwise = deque(clockwise + list(extracups))



for i in range(rounds):
    if not i % 10000:
        print(f"round {i+1}")
    current = clockwise.popleft()
    p = []
    p.append(clockwise.popleft())
    p.append(clockwise.popleft())
    p.append(clockwise.popleft())
    d = dest(p, current, size)
    di = len(clockwise) - 1
    for c in reversed(clockwise):
        if c == d:
            break
        di = di - 1

    clockwise.insert(di+1, p[2])
    clockwise.insert(di+1, p[1])
    clockwise.insert(di+1, p[0])
    clockwise.append(current)
    
one = clockwise.index(1)
out = []
for i in range(one+1, one+3):
    out.append(clockwise[i%len(clockwise)])
print(out)
