#!/usr/bin/env python

import sys

prev_accumulator = 0
accumulator = 0
pointer = 0

def nop(arg):
    global pointer
    pointer = pointer + 1

def acc(arg):
    global pointer
    global accumulator
    accumulator = accumulator + arg
    pointer = pointer + 1

def jmp(arg):
    global pointer
    pointer = pointer + arg

functions = {
    'nop': nop,
    'acc': acc,
    'jmp': jmp,
}

lines = [l.rstrip() for l in sys.stdin.readlines()]

code = []

for l in lines:
    cmd, arg = l.split(' ')
    code.append((cmd, int(arg), 0))

while all([c < 2 for (_,_,c) in code]):
    prev_accumulator = accumulator
    print('running')
    cur_pointer = pointer
    cmd, arg, c = code[cur_pointer]
    functions[cmd](arg)
    code[cur_pointer] = (cmd, arg, c + 1)

print(prev_accumulator)
