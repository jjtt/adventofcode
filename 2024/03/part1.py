#!/usr/bin/env python

import re

pattern = r"mul\((\d+),(\d+)\)"

sum = 0
for line in open("input.txt"):
    matches = re.findall(pattern, line)
    for match in matches:
        sum = sum + int(match[0]) * int(match[1])

print(sum)
