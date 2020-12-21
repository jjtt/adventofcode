#!/usr/bin/env python

import sys
import itertools
import pprint

lines = [l.rstrip() for l in sys.stdin.readlines()] 

foods = []

for l in lines:
    i,a = l.split(' (contains ')
    foods.append((tuple(i.split(' ')), tuple(a[:-1].split(', '))))

allergens = set()
for _,alist in foods:
    for a in alist:
        allergens.add(a)

ingredients = set()
for ilist,_ in foods:
    for i in ilist:
        ingredients.add(i)

ok = ingredients
fixed = {}
for a in allergens:
    ing = ingredients
    for i,alist in foods:
        if a in alist:
            ing = ing & set(i)
    fixed[a] = ing
    ok = ok - ing

tankerot = []

while len(fixed) > 1:
    for a,candidates in fixed.items():
        if len(candidates) == 1:
            fix = candidates.pop()
            tankerot.append((a,fix))
            fixed.pop(a)
            for o,ocand in fixed.items():
                fixed[o] = ocand - set([fix])
            break
a,fix = fixed.popitem()
tankerot.append((a,fix.pop()))

tankerot.sort()

print(','.join([t for _,t in tankerot]))
