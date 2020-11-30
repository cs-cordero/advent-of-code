#!/bin/python3

import re


def decompress_helper(s):
    if s[0] == "(":
        pass


def decompress(s):
    m = re.match(r"^(\((\d+)x(\d+)\))", s)
    s = s[len(m.group(0)) :]
    if s[0] == "(":
        s = decompress_helper(s)
    s = s[len(m.group(1)) + int(m.group(2)) :]
    return s, int(m.group(2)) * int(m.group(3))


def count_them(s):
    counter = 0
    while s:
        if s[0] == "(":
            dec = decompress(s)
            s = dec[0]
            counter += dec[1]
            continue
        m = re.search(r"\(", s)
        if not m:
            counter += len(s)
            break
        counter += len(s[: m.start()])
        s = s[m.start() :]
    return counter


def load():
    with open("aoc_day_09_input.txt", "r") as f:
        ins = "".join(re.sub(r"\s*", r"", line) for line in f)
    return count_them(ins)


print("Sample 1: %d" % count_them("(3x3)XYZ"))
print("Sample 2: %d" % count_them("X(8x2)(3x3)ABCY"))
print("Sample 3: %d" % count_them("(27x12)(20x12)(13x14)(7x10)(1x12)A"))
print(
    "Sample 4: %d"
    % count_them("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
)
# print('Challenge: %d' % load())
