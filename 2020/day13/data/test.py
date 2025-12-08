#!/usr/bin/env python3

import sys
from functools import reduce


with open(sys.argv[1], "r") as f:
    start = int(f.readline())
    buses_raw = f.readline().strip().split(",")

buses = [int(b) for b in buses_raw if b != "x"]
waits = [(b, b - (start % b)) for b in buses]

answer = min(waits, key=lambda x: x[1])

print(f"Part 1: {answer[0] * answer[1]}")


def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a * b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1:
        return 1
    while a > 1:
        q = a // b
        a, b = b, a % b
        x1, x1 = x1 - q * x0, x0
    if x1 < 0:
        x1 += b0
    return x1


offsets = [int(b) - i for i, b in enumerate(buses_raw) if b != "x"]
print(f"Part 2: {chinese_remainder(buses, offsets)}")
