#!/usr/bin/env python

import sys

nums = [int(l.rstrip()) for l in sys.stdin.readlines()]

nums.append(0)
nums.append(max(nums) + 3)

nums.sort()

prev = None

diffs = {}

for n in nums:
    if not prev is None:
        diff = n - prev
        count = diffs.get(diff, 0)
        diffs[diff] = count + 1
    prev = n

print(diffs[1]*diffs[3])
