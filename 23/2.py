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
    print(f"round {i+1}")
    print(clockwise)
    current = clockwise.popleft()
    clockwise.append(current)
    p = []
    p.append(clockwise.popleft())
    p.append(clockwise.popleft())
    p.append(clockwise.popleft())
    print(p)
    d = dest(p, current, size)
    print(d)

    nextstart = deque([])
    c = clockwise.popleft()
    while not c == d:
        nextstart.append(c)
        c = clockwise.popleft()

    clockwise.appendleft(p[2])
    clockwise.appendleft(p[1])
    clockwise.appendleft(p[0])
    clockwise.appendleft(c)

    nextstart += clockwise
    clockwise = nextstart
    
one = clockwise.index(1)
out = []
for i in range(one+1, one+3):
    out.append(clockwise[i%len(clockwise)])
print(out)

m = 0
c = 0
for i in range(1, size):
    if clockwise[i-1]+1 == clockwise[i]:
        c += 1
    else:
        m = max(m, c)
        c = 0
print(max(m,c))

