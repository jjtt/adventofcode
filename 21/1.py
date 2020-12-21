#!/usr/bin/env python

import sys
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

ing_by_al = {}
for i,a in foods:
    for al in a:
        for ing in i:
            print(f"{al}({ing}).")

        #print(f"{al}({al.upper()}).")
    

