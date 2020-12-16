#!/usr/bin/env python

import sys
import math

def test_class(v, tests):
    min1, max1 = tests[0]
    min2, max2 = tests[1]

    return (int(min1) <= v <= int(max1)) or (int(min2) <= v <= int(max2))

def test_ticket(t, testlist):
    valid_for_some = set()
    for v in t:
        value = int(v)
        for tests in testlist:
            if test_class(value, tests):
                valid_for_some.add(value)
    invalid = set([int(v) for v in t]) - valid_for_some
    return sum(invalid) > 0
    

lines = [l.rstrip() for l in sys.stdin.readlines()]

classes = {}
ticket = None
tickets = []

mode = 0
for l in lines:
    if l == '':
        pass
    elif l in ['your ticket:', 'nearby tickets:']:
        mode = mode + 1
    elif mode == 0:
        k,v = l.split(':')
        classes[k] = [c.split('-') for c in v.strip().split(' or ')]
    elif mode == 1:
        ticket = l.split(',')
    else:
        tickets.append(l.split(','))

valids = [t for t in tickets if not test_ticket(t, classes.values())]

candidates = {k: list(range(len(ticket))) for k in classes.keys()}

for field in range(len(ticket)):
    for t in valids: # + [ticket]:
        for k in classes.keys():
            if not test_class(int(t[field]), classes[k]):
                if field in candidates[k]:
                    candidates[k].remove(field)

print(candidates)

printed = []
fields = {}
while len(candidates) > 0:
    newcand = {}
    for k in candidates.keys():
        if len(candidates[k]) == 1:
            printed.append(candidates[k][0])
            fields[k] = candidates[k][0]
            print((k, candidates[k][0]))
        else:
            newcand[k] = [v for v in candidates[k] if (v not in printed)]
    candidates = newcand

        
values = []
for k in fields.keys():
    if k.startswith('departure'):
        print((k,ticket[fields[k]]))
        values.append(int(ticket[fields[k]]))
print(math.prod(values))
