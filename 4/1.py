#!/usr/bin/env python

import sys

fields = set([
    'byr',
    'iyr',
    'eyr',
    'hgt',
    'hcl',
    'ecl',
    'pid',
#    'cid'
])

lines = [l.rstrip() for l in sys.stdin.readlines()]

def todict(one):
    j = " ".join(one)
    return {k: v for (k, v) in [p.split(":") for p in j.split(" ")]}

def isvalid(d):
    return not fields - set(d.keys())

count = 0
one = []
for l in lines + [""]:
    if l == "":
        if isvalid(todict(one)):
            count = count + 1
        one = []
    else:
       one.append(l)

print(count)
