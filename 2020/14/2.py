#!/usr/bin/env python

import sys

def print_mask():
    global mask_one
    global mask_float

    print(bin(mask_one))
    print(bin(mask_float))

def update_mask(mask):
    global mask_one
    global mask_float

    mask_one = 0
    mask_float = 0

    for i in range(len(mask)):
        mask_one = mask_one << 1
        mask_float = mask_float << 1

        if mask[i] == '1':
            mask_one = mask_one | 0b1
        if mask[i] == 'X':
            mask_float = mask_float | 0b1

def floating(addr):
    global mask_float

    addresses = [addr]
    for i in range(36):
        if (mask_float >> i) & 0b1 == 0b1:
            a2 = []
            for a in addresses:
                a2.append(a | (0b1 << i))
                a2.append(a & ~(0b1 << i))
            addresses = a2
    return addresses

def update_mem(addr, val):
    global mask_one
    global mem

    addr = addr | mask_one
    for a in floating(addr):
        mem[a] = val

lines = [l.rstrip() for l in sys.stdin.readlines()]

mem = {}

for c,v in [l.split(' = ') for l in lines]:
    if c == 'mask':
        update_mask(v)
    else:
        update_mem(int(c[4:-1]), int(v))

print(sum(mem.values()))
