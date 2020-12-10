#!/usr/bin/env python

import sys
import math

def drop_comb(l):
    if l < 3:
        return pow(2,l)
    elif l == 3:
        return 7

nums = [int(l.rstrip()) for l in sys.stdin.readlines()]

nums.append(0)
nums.append(max(nums) + 3)

nums.sort()

prev = None

skippable = [[]]

group = 0

prev = nums[0]
for n in nums:
    if n - 1 == prev:
        skippable[group].append(n)
    else:
        skippable.append([])
        group = group+1
    prev = n

skippable = [s[:-1] for s in skippable if len(s) > 1]

print(skippable)
print(max(map(len,skippable)))

print(math.prod([drop_comb(len(s)) for s in skippable]))
