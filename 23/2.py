#!/usr/bin/env python

import sys
import numpy as np

def dest(picked, current, size):
    while True:
        current = (current - 1) % size
        if current == 0:
            current = size
        if current not in picked:
            return current

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

indices = np.array(list(range(size+1)))
for i in range(len(clockwise)):
    indices[clockwise[i]] = i

head = clockwise[:4]

nums = np.arange(indices.size)
nums[0] = -42
indices[0] = -42

for i in range(rounds):
    if not i % 10000:
        print(f"round {i+1}")
    #print(f"round {i+1}")
    #print(to_circle(indices))
    #print(head)
    d = dest(head[1:], head[0], size)
    #print(d)
    di = indices[d]


    indices = np.select(
        [
            nums == -42,
            nums == head[0],
            np.logical_and(np.isin(nums,head,invert=True),indices<=di),
            np.logical_and(np.isin(nums,head,invert=True),indices>di),
            True,
        ],
        [
            nums,
            np.full_like(indices, size-1),
            (indices - 4) % size,
            (indices - 1) % size,
            (indices + di-4) % size,
        ]
    )
    head = np.concatenate((nums[indices==0], nums[np.logical_and(indices<4, indices>0)]))
    
clockwise = to_circle(indices)
one = clockwise.index(1)
out = []
for i in range(one+1, one+3):
    out.append(clockwise[i%len(clockwise)])
print(out)

print(out[0] * out[1])
