#!/usr/bin/env python

import sys

def print_mask():
    global mask_or
    global mask_and

    print(bin(mask_or))
    print(bin(mask_and))

def update_mask(mask):
    global mask_or
    global mask_and

    mask_or = 0
    mask_and = 0

    for i in range(len(mask)):
        mask_and = mask_and << 1
        mask_and = mask_and | 0b1
        mask_or = mask_or << 1

        if mask[i] == '0':
            mask_and = mask_and & 0b111111111111111111111111111111111110
        if mask[i] == '1':
            mask_or = mask_or | 0b1

def update_mem(addr, val):
    val = val & mask_and
    val = val | mask_or
    mem[addr] = val

lines = [l.rstrip() for l in sys.stdin.readlines()]

mem = {}

for c,v in [l.split(' = ') for l in lines]:
    if c == 'mask':
        update_mask(v)
    else:
        update_mem(int(c[4:-1]), int(v))

print(sum(mem.values()))
