#!/usr/bin/env python

import sys

def is_sum(n, pre):
    for p1 in pre:
        for p2 in pre:
            if n == p1 + p2:
                return True
    return False

nums = [int(l.rstrip()) for l in sys.stdin.readlines()]

presize = int(sys.argv[1])

pre = nums[:presize]

weak = -1

for n in nums[presize:]:
    if not is_sum(n, pre):
        weak = n
        break
    pre.pop(0)
    pre.append(n)

summed = []

for n in nums:
    if sum(summed) < weak:
        summed.append(n)
    if sum(summed) == weak:
        print(summed[0])
        print(summed[-1])
        break
    if sum(summed) > weak:
        summed.pop(0)
