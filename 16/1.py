#!/usr/bin/env python

import sys

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
    return sum(invalid)
    

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

print(sum([test_ticket(t, classes.values()) for t in tickets]))
