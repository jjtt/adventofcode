#!/usr/bin/env python
import sys

digits =  '0123456789ABCDEFGHIJKLMNOP'
letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'

def to_pw(number):
    """Adapted from numpy"""
    base = len(letters)

    num = abs(number)
    res = []
    while num:
        res.append(digits[num % base])
        num //= base
    if number < 0:
        res.append('-')
    pw = ''.join(reversed(res or '0'))
    for i in reversed(range(len(letters))):
        pw = pw.replace(digits[i], letters[i])
    return pw

def to_num(pw):
    pw = pw.upper()
    for i in range(len(letters)):
        pw = pw.replace(letters[i], digits[i])
    return int(pw, len(letters))

def valid(pw):
    pw = pw.upper()
    for i in range(len(letters)-2):
        if letters[i:i+3] in pw:
            break
    else:
        return False

    if any(c in pw for c in "IOL"):
        return False

    return sum([c+c in pw for c in letters]) >= 2

def next_valid(number):
    while True:
        number += 1
        if valid(to_pw(number)):
            return number

assert to_num('A') == 0
assert to_num('Z') == 25
assert to_num('ZZ') == 25 * 26 ** 1 + 25
assert to_num('ZZA') == 25 * 26 ** 2 + 25 * 26 ** 1

assert to_pw(0) == 'A'
assert to_pw(25) == 'Z'
assert to_pw(25 * 26 ** 1 + 25) == 'ZZ'
assert to_pw(25 * 26 ** 2 + 25 * 26 ** 1) == 'ZZA'

assert not valid("hijklmmn")
assert not valid("abbceffg")
assert not valid("abbcegjk")
assert not valid("abbcefgj")

assert next_valid(to_num("abcdefgh")) == to_num("abcdffaa")
assert next_valid(to_num("ghijklmn")) == to_num("ghjaabcc")

print(to_pw(next_valid(to_num("cqjxjnds"))).lower())
