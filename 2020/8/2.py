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

def uncorrupt(code, i):
    cmd,arg,c = code[i]
    if cmd == 'nop':
        cmd = 'jmp'
    elif cmd == 'jmp':
        cmd = 'nop'
    code = code.copy()
    code[i] = (cmd,arg,c)
    return code

lines = [l.rstrip() for l in sys.stdin.readlines()]

code = []

for l in lines:
    cmd, arg = l.split(' ')
    code.append((cmd, int(arg), 0))

try:
    for i in range(len(code)):
        
        newcode = uncorrupt(code, i)
        accumulator = 0
        pointer = 0
        while all([c < 2 for (_,_,c) in newcode]):
            prev_accumulator = accumulator
            print('running')
            cur_pointer = pointer
            cmd, arg, c = newcode[cur_pointer]
            functions[cmd](arg)
            newcode[cur_pointer] = (cmd, arg, c + 1)

        print(f'infinite loop: ${prev_accumulator}')
except IndexError:
    print(accumulator)
