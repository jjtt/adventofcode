#!/usr/bin/env python

import sys

def handle_list(rules, rule):
    return ''.join([to_regexp(rules, a) for a in rule.split(' ')]) 

def to_regexp(rules, r):
    rule = rules[r]
    if rule.startswith('"'):
        return rule.replace('"','')
    if '|' in rule:
        r1, r2 = rule.split(' | ')
        return f"({handle_list(rules, r1)}|{handle_list(rules, r2)})"
    return handle_list(rules, rule)

lines = [l.rstrip() for l in sys.stdin.readlines()] 

rules = {}

for l in lines:
    if l == "":
        print(to_regexp(rules, '0'))
        break
    else:
        n,r = l.split(': ')
        rules[n] = r
