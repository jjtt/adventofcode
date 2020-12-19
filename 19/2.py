#!/usr/bin/env python

import sys

def handle_list(rules, rule, visited):
    return ''.join([to_regexp(rules, a, visited) for a in rule.split(' ')]) 

def to_regexp(rules, r, visited):
    if r == '11':
        #return '(((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)**42 31 | 42 11 31**(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a))))'
        #return '(((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)**11**(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a))))'

        return '(((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)((((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))))?(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a))))'

    if r == '8':
        #return '(((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)**42 | 42 8**)'
        return '(((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)|((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a))+'
    if r == '42':
        return '((b(a(bb|ab)|b(a|b)(a|b))|a(bbb|a(bb|a(a|b))))b|(((aa|ab)a|bbb)b|((a|b)a|bb)aa)a)'
    if r == '31':
        return '(b(b(aba|baa)|a(b(ab|(a|b)a)|a(ba|ab)))|a(b((ab|(a|b)a)b|((a|b)a|bb)a)|a(bab|(ba|bb)a)))'
    rule = rules[r]
    if rule.startswith('"'):
        out = rule.replace('"','')
    elif r in visited:
        out = f'*{rule}*'
    else:
        visited = visited + [r]
        if '|' in rule:
            r1, r2 = rule.split(' | ')
            out = f"({handle_list(rules, r1, visited)}|{handle_list(rules, r2, visited)})"
        else:
            out = handle_list(rules, rule, visited)
    #return f"({r}:{out})"
    return out

lines = [l.rstrip() for l in sys.stdin.readlines()] 

rules = {}

for l in lines:
    if l == "" or not ':' in l:
        continue
    else:
        n,r = l.split(': ')
        rules[n] = r

import json
#print(json.dumps(rules, indent=4, sort_keys=True))
#print(0)
print(to_regexp(rules, '0',[]))
#print(42)
#print(to_regexp(rules, '42',[]))
#print(8)
#print(to_regexp(rules, '8',[]))
#print(11)
#print(to_regexp(rules, '11',[]))
#print(31)
#print(to_regexp(rules, '31',[]))
