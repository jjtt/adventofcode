#!/usr/bin/env python
import sys
import re
import itertools
import random
import math

pattern = re.compile(
    "^(.*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$"
)


def parse(lines):
    return list(
        map(
            lambda m: (
                m.group(1),
                int(m.group(2)),
                int(m.group(3)),
                int(m.group(4)),
                int(m.group(5)),
                int(m.group(6)),
            ),
            map(lambda line: pattern.match(line), lines),
        )
    )


def mutate(orig):
    recipe = dict(orig)
    jump = random.choice(range(1, 26))
    ingredients = set(recipe.keys())
    choises = list(ingredients)
    while True:
        minus = random.choice(choises)
        if recipe[minus] > jump:
            break
    choises = list(ingredients.difference([minus]))
    while True:
        plus = random.choice(choises)
        if recipe[plus] < 100 - jump:
            break
    recipe[minus] = recipe[minus] - jump
    recipe[plus] = recipe[plus] + jump
    return recipe


def value(recipe, ingredients):
    return math.prod(
        [
            s if s > 0 else 0
            for s in [
                sum([i[p] * recipe[i[0]] for i in ingredients]) for p in range(1, 5)
            ]
        ]
    )


f = open(sys.argv[1])

ingredients = parse(f.readlines())

print(ingredients)

(start, _, _, _, _, _) = random.choice(ingredients)
recipe = {i: 100 if i == start else 0 for (i, _, _, _, _, _) in ingredients}
print(recipe)

v = value(recipe, ingredients)
print(v)

TRIES = 400
nochange = TRIES
while nochange > 0:
    nochange -= 1
    recipe2 = mutate(recipe)
    print(recipe2)
    v2 = value(recipe2, ingredients)
    print(v2)
    if v2 >= v:
        print("found better")
        recipe = recipe2
        v = v2
        nochange = TRIES


print(recipe)
print(v)
