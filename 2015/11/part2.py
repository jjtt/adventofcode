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


print(to_pw(next_valid(to_num("cqjxxyzz"))).lower())
