#!/usr/bin/env python
import sys


def play(line):
    new = ""
    start = 0
    for i in range(1, len(line)):
        if line[i] != line[i-1]:
            new += str(i-start)
            new += line[start]
            start = i
    return new + str(len(line) - start) + line[start]


assert play("1") == "11"
assert play("11") == "21"
assert play("21") == "1211"
assert play("1211") == "111221"
assert play("111221") == "312211"

part1 = "1113222113"
for _ in range(40):
    part1 = play(part1)
print(len(part1))
