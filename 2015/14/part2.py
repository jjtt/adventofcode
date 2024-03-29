#!/usr/bin/env python
import sys
import re
import itertools

pattern = re.compile(
    "^(.*) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$"
)


def parse(lines):
    return list(
        map(
            lambda m: (m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4))),
            map(lambda line: pattern.match(line), lines),
        )
    )


def fly(speed, move, rest, time):
    if (time - 1) % (move + rest) < move:
        return speed
    else:
        return 0


f = open(sys.argv[1])

reindeer = parse(f.readlines())

pos = {name: 0 for (name, _, _, _) in reindeer}
points = {name: 0 for name in pos.keys()}

for t in range(1, int(sys.argv[2]) + 1):
    for (n, s, m, r) in reindeer:
        pos[n] = pos[n] + fly(s, m, r, t)
    lead = max(pos, key=pos.get)
    lead_dist = pos[lead]
    for (n, d) in pos.items():
        if d == lead_dist:
            points[n] = points[n] + 1


winner = max(points, key=points.get)
print(f"t={t}: {winner} {points[winner]}")
