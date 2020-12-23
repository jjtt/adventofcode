#!/usr/bin/env python

import sys

def dest(picked, current, size):
    while True:
        current = (current - 1) % size
        if current == 0:
            current = size
        if current not in picked:
            return current

def print_circle(indices):
    print(indices)
    d = {indices[v]: v for v in range(1,size+1)}
    print([d[i] for i in sorted(d.keys())])

def to_circle(indices):
    d = {indices[v]: v for v in range(1,size+1)}
    return [d[i] for i in sorted(d.keys())]

rounds = int(sys.argv[1])

lines = [l.rstrip() for l in sys.stdin.readlines()] 

clockwise = [int(i) for i in lines[0]]

size = int(sys.argv[2])
cupcount = size - len(clockwise)

extracups = range(max(clockwise)+1, max(clockwise)+cupcount+1)

clockwise = clockwise + list(extracups)

indices = list(range(size+1))
for i in range(len(clockwise)):
    indices[clockwise[i]] = i

head = clockwise[:4]

print(clockwise)
print(indices)
print(to_circle(indices))
print(head)



for i in range(rounds):
    if not i % 10000:
        print(f"round {i+1}")
    print(f"round {i+1}")
    print(to_circle(indices))
    print(head)
    d = dest(head[1:], head[0], size)
    print(d)
    di = indices[d]
    newhead = [0] * 4
    for n in range(1, size+1):
        if n == head[0]:
            indices[n] = size-1
        elif n not in head:
            if indices[n] <= di:
                indices[n] = (indices[n] - 4) % size
            else:
                indices[n] = (indices[n] - 1) % size
        else:
            indices[n] = (indices[n] + (di-4)) % size
        
        if indices[n] < 4:
            newhead[indices[n]] = n

    head = newhead
    
clockwise = to_circle(indices)
one = clockwise.index(1)
out = []
for i in range(one+1, one+3):
    out.append(clockwise[i%len(clockwise)])
print(out)

print(out[0] * out[1])
