#!/usr/bin/env python

import sys
import itertools
import pprint

def remove_fixed(ing_by_al, a, i):
    otherals = set(ing_by_al.pop(a)) - set(i)
    new_ing_by_al = {}
    for a2,i2 in ing_by_al.items():
        if a == a2:
            continue
        new_ing_by_al[a2] = list(set(i2) - set(i))
    return new_ing_by_al
            
        

lines = [l.rstrip() for l in sys.stdin.readlines()] 

foods = []

for l in lines:
    i,a = l.split(' (contains ')
    foods.append((tuple(i.split(' ')), tuple(a[:-1].split(', '))))

facts = []
allergens = set()
for i,a in foods:
    for al in a:
        allergens.add(al)
        for ing in i:
            facts.append(f"{al}({ing}).")
facts.sort()

for f in facts:
    print(f)

print("solve(", ','.join([a.upper() for a in allergens]), "):-", ', '.join([f"{a}({a.upper()})" for a in allergens] + [f"not({a.upper()}={b.upper()})" for a,b in itertools.combinations(allergens, 2)]), ".")

    

