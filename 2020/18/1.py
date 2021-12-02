#!/usr/bin/env python

import sys

def find_first(s):
    has_paren = s[0] == '('
    open_paren = 0
    for i in range(len(s)):
        if s[i] == ' ':
            if not has_paren:
                return s[:i], s[i+1:]
            elif open_paren == 0:
                return cal(s[1:i-1]), s[i+1:]
        if s[i] == '(':
            open_paren = open_paren + 1
        if s[i] == ')':
            open_paren = open_paren - 1
    if has_paren:
        return cal(s[1:-1]), None
    else:
        return s, None

def find_parts(s):
    n1,rest = find_first(s)
    if rest is None:
        return n1, None, None, None
    op,rest = find_first(rest)
    n2,rest = find_first(rest)

    return n1, op, n2, rest
    

def cal(s):
    n1, op, n2, rest = find_parts(s)

    if op == '+':
        r = int(n1) + int(n2)
    elif op == '*':
        r = int(n1) * int(n2)
    else:
        r = int(n1)

    if rest is not None:
        return cal(f"{r} {rest}") 
    else:
        return str(r)

lines = [l.rstrip() for l in sys.stdin.readlines()] 

for l in lines:
    print(cal(l))
