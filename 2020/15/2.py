#!/usr/bin/env python

import sys

def say(n):
    global last
    global before
    global turn

    if n in last:
        before[n] = last[n]
    last[n] = turn
    return n

lines = [l.rstrip() for l in sys.stdin.readlines()]

start = [int(s) for s in lines[0].split(',')]

turn = 1

last = {}
before = {}
prev = None

for n in start:
    say(n)
    turn = turn + 1
    prev = n

while turn <= 30000000:
    if not prev in before:
        prev = say(0)
    else:
        prev = say(last[prev] - before[prev])
    turn = turn + 1

print(prev)
    
        
        
        
