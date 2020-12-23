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
    current = clockwise.pop(0)
    clockwise.append(current)
    p = clockwise[:3]
    del clockwise[:3]
    d = dest(p, current, size)

    ci = 0
    while not clockwise[ci] == d:
        ci += 1

    nextstart = clockwise[:ci+1]
    del clockwise[:ci+1]

    clockwise[0:0] = p

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

