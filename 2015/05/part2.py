#!/usr/bin/env python
import sys
from collections import Counter


def nice(s):
    pairs = set([s[i : i + 2] for i in range(len(s) - 1)])

    found_pair = False
    for pair in pairs:
        if pair in s.split(pair, 1)[1]:
            found_pair = True
            break
    if not found_pair:
        return False

    for i in range(len(s) - 2):
        if s[i] == s[i + 2]:
            return True

    return False


print(nice("qjhvhtzxzqqjkmpb"))
print(nice("xxyxx"))
print(not nice("uurcxstgmygtbstg"))
print(not nice("ieodomkazucvgmuy"))
print(not nice("aaa"))

f = "input.txt"

if len(sys.argv) > 1:
    f = sys.argv[1]

nices = 0
for s in open(f).readlines():
    if nice(s.strip()):
        nices += 1

print(nices)
