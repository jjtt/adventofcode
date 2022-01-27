#!/usr/bin/env python
import sys


def nice(s):
    for nono in ["ab", "cd", "pq", "xy"]:
        if nono in s:
            return False

    if 3 > len("".join([c for c in s if c in "aeiou"])):
        return False

    for i in range(len(s) - 1):
        if s[i] == s[i + 1]:
            return True

    return False


print(nice("ugknbfddgicrmopn"))
print(nice("aaa"))
print(not nice("jchzalrnumimnmhp"))
print(not nice("haegwjzuvuyypxyu"))
print(not nice("dvszwmarrgswjxmb"))

f = "input.txt"

if len(sys.argv) > 1:
    f = sys.argv[1]

nices = 0
for s in open(f).readlines():
    if nice(s):
        nices += 1

print(nices)
