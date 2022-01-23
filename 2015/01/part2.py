#!/usr/bin/env python

line = open("input.txt").readlines()[0].strip()

floor = 0
for i in range(len(line)):
    if line[i] == "(":
        floor += 1
    elif line[i] == ")":
        floor -= 1

    if floor < 0:
        print(i + 1)
        exit()
