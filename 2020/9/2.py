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

s1 = 0
s2 = 0

while(True):
    if sum(nums[s1:s2]) < weak:
        s2 = s2 + 1
    if sum(nums[s1:s2]) == weak:
        l = nums[s1:s2]
        l.sort()
        print(l[0] + l[-1])
        break
    if sum(nums[s1:s2]) > weak:
        s1 = s1 + 1
