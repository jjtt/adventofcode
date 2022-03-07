#!/usr/bin/env python
import sys
import re
import itertools

pattern = re.compile(
    "^(.*) would (gain|lose) (\d+) happiness units by sitting next to (.*)\.$"
)


def parse(lines):
    happies = {}
    for line in lines:
        m = pattern.match(line)
        p = happies.get(m.group(1), {})
        val = int(m.group(3))
        p[m.group(4)] = val if m.group(2) == "gain" else -val
        happies[m.group(1)] = p
    return happies


f = open(sys.argv[1])

happies = parse(f.readlines())
names = list(happies.keys())

perms = [
    [names[0]] + list(p) for p in itertools.permutations(names[1:]) if p[0] <= p[-1]
]

print(
    max(
        [
            sum(
                [
                    happies[p[i]][p[i - 1]] + happies[p[i]][p[(i + 1) % len(names)]]
                    for i in range(len(names))
                ]
            )
            for p in perms
        ]
    )
)
