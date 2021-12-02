#!/usr/bin/env python

import sys
import re

def toint(x):
    try:
        return int(x)
    except ValueError:
        return None

def byr(x):
    year = toint(x)
    return year >= 1920 and year <= 2002

def iyr(x):
    year = toint(x)
    return year >= 2010 and year <= 2020

def eyr(x):
    year = toint(x)
    return year >= 2020 and year <= 2030

def hgt(x):
    h = toint(x[:-2])
    if x[-2:] == "cm":
        return h >= 150 and h <= 193
    if x[-2:] == "in":
        return h >= 59 and h <= 76
    return False

def hcl(x):
    return re.match(r"#[0-9a-f]{6}$", x)

def ecl(x):
    return x in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

def pid(x):
    return re.match(r"[0-9]{9}$", x)

fields = {
    'byr': byr,
    'iyr': iyr,
    'eyr': eyr,
    'hgt': hgt,
    'hcl': hcl,
    'ecl': ecl,
    'pid': pid,
}

lines = [l.rstrip() for l in sys.stdin.readlines()]

def todict(one):
    j = " ".join(one)
    return {k: v for (k, v) in [p.split(":") for p in j.split(" ")]}

def isvalid(d):
    ok = not set(fields.keys()) - set(d.keys())
    for k in fields.keys():
        if k in d:
            ok = ok and fields[k](d[k])
    return ok

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
