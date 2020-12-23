#!/usr/bin/env python

import sys

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

clockwise = clockwise + list(extracups)


for i in range(rounds):
    if not i % 10000:
        print(f"round {i+1}")
    print(f"round {i+1}")
    print(clockwise)
    current = clockwise.pop(0)
    clockwise.append(current)
    p = []
    p.append(clockwise.pop(0))
    p.append(clockwise.pop(0))
    p.append(clockwise.pop(0))
    print(p)
    d = dest(p, current, size)
    print(d)

    nextstart = []
    c = clockwise.pop(0)
    while not c == d:
        nextstart.append(c)
        c = clockwise.pop(0)

    clockwise.insert(0, p[2])
    clockwise.insert(0, p[1])
    clockwise.insert(0, p[0])
    clockwise.insert(0, c)

    clockwise[0:0] = nextstart
    
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

