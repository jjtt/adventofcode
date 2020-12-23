#!/usr/bin/env python

import sys

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

clockwise = clockwise + list(extracups)

ci = 0


for i in range(rounds):
    if not i % 10000:
        print(f"round {i+1}")
    p, end = pick(clockwise, ci+1, 3)
    d = dest(p, clockwise[ci], size)

    move = ci + 4
    clockwise[(move-3)%size] = clockwise[move%size]
    while not clockwise[move%size] == d:
        move = move + 1
        clockwise[(move-3)%size] = clockwise[move%size]
    clockwise[(move-2)%size] = p[0]
    clockwise[(move-1)%size] = p[1]
    clockwise[(move)%size] = p[2]
        
    ci = (ci + 1) % size
    
one = clockwise.index(1)
out = []
for i in range(one+1, one+3):
    out.append(clockwise[i%len(clockwise)])
print(out)
