#!/usr/bin/env python
import sys


def parse(line):
    s = line.strip().split(" ")
    if "AND" in line:
        return (s[4], (lambda x, y: x & y, [s[0], s[2]]))
    elif "OR" in line:
        return (s[4], (lambda x, y: x | y, [s[0], s[2]]))
    elif "LSHIFT" in line:
        return (s[4], (lambda x, y: x << y, [s[0], s[2]]))
    elif "RSHIFT" in line:
        return (s[4], (lambda x, y: x >> y, [s[0], s[2]]))
    elif "NOT" in line:
        return (s[3], (lambda x: (~x + 65536) % 65535, [s[1]]))
    else:
        return (s[2], (None, [s[0]]))


def evaluate(gates, wire):
    if not wire in gates:
        value = int(wire)
    else:
        (op, expressions) = gates[wire]
        if op is None:
            assert len(expressions) == 1
            if type(expressions[0]) == str:
                value = evaluate(gates, expressions[0])
            else:
                value = expressions[0]
        else:
            value = op(*[evaluate(gates, x) for x in expressions])
    gates[wire] = (None, [value])
    return value


f = "input.txt"

if len(sys.argv) > 1:
    f = sys.argv[1]

gates = {}
for line in open(f).readlines():
    (wire, (op, expressions)) = parse(line)
    gates[wire] = (op, expressions)

if f == "sample1.txt":
    assert 123 == evaluate(gates, "x")
    assert 456 == evaluate(gates, "y")
    assert 72 == evaluate(gates, "d")
    assert 507 == evaluate(gates, "e")
    assert 492 == evaluate(gates, "f")
    assert 114 == evaluate(gates, "g")
    assert 65412 == evaluate(gates, "h")
    assert 65079 == evaluate(gates, "i")
    assert 123 == evaluate(gates, "foobar")
elif f == "input.txt":
    assert 16076 == evaluate(gates, "a")
