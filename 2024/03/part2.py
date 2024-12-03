#!/usr/bin/env python

import re

pattern = r"mul\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)"

sum = 0
for line in open("input.txt"):
    matches = re.findall(pattern, line)
    do = True
    for match in matches:
        if match[2] == "do":
            do = True
        elif match[3] == "don't":
            do = False
        elif do:
            sum = sum + int(match[0]) * int(match[1])

print(sum)
