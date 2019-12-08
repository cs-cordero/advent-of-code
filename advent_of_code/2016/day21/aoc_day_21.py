#!/bin/python3

import re

SCRAMBLE = "abcdefgh"
SCRAMBLE2 = "fbgdceah"
# SCRAMBLE2 = 'gbhafcde'
# SCRAMBLE = 'abcde'
# SCRAMBLE2 = 'decab'
S_LEN = len(SCRAMBLE)
# S_LEN = 8


def swappos(s, x, y):
    x, y = map(int, (x, y))
    s[x], s[y] = s[y], s[x]
    return s


def swapltr(s, a, b):
    for i in range(S_LEN):
        if s[i] == a:
            x = i
        elif s[i] == b:
            y = i
    return swappos(s, x, y)


def rotate(s, steps, direction="right"):
    steps = int(steps) % S_LEN
    steps *= -1 if direction == "right" else 1
    s = s[steps:] + s[:steps]
    return s


def rotate_on_ltr(s, x, rev="right"):
    for i in range(S_LEN):
        if s[i] == x:
            steps = i
            break
    steps += 2 if steps >= 4 else 1
    steps %= S_LEN

    undo = {
        0: (1, "left"),
        1: (1, "left"),
        2: (2, "right"),
        3: (2, "left"),
        4: (1, "right"),
        5: (3, "left"),
        6: (0, "right"),
        7: (4, "right"),
    }
    if rev == "left":
        steps, rev = undo[i]
    return rotate(s, steps, rev)


def move(s, x, y):
    x, y = map(int, (x, y))
    s.insert(y, s.pop(x))
    return s


def reverse_s(s, x, y):
    x, y = map(int, (x, y))
    s = s[:x] + list(reversed(s[x : y + 1])) + s[y + 1 :]
    return s


def main():
    s = [a for a in SCRAMBLE]
    t = [a for a in SCRAMBLE2]

    rotateltr = re.compile(r"letter (\w)")
    rotatenum = re.compile(r"rotate (left|right) (\d)")
    swap_ltr = re.compile(r"letter (\w) with letter (\w)")
    swap_pos = re.compile(r"position (\w) with position (\w)")
    rev_pos = re.compile(r"(\d) through (\d)")
    move_pos = re.compile(r"position (\w) to position (\w)")

    with open("aoc_day_21_input.txt") as f:
        inst_stack = f.readlines()

    for line in inst_stack:
        if line.startswith("rotate based"):
            letter = rotateltr.search(line).group(1)
            s = rotate_on_ltr(s, letter)
        elif line.startswith("rotate"):
            d, n = rotatenum.search(line).groups()
            s = rotate(s, n, d)
        elif line.startswith("swap letter"):
            a, b = swap_ltr.search(line).groups()
            s = swapltr(s, a, b)
        elif line.startswith("swap position"):
            x, y = swap_pos.search(line).groups()
            s = swappos(s, x, y)
        elif line.startswith("reverse positions"):
            x, y = rev_pos.search(line).groups()
            s = reverse_s(s, x, y)
        elif line.startswith("move position"):
            x, y = move_pos.search(line).groups()
            s = move(s, x, y)

    while inst_stack:
        line = inst_stack.pop()
        if line.startswith("rotate based"):
            letter = rotateltr.search(line).group(1)
            print("trying to rotate on %s" % letter)
            t = rotate_on_ltr(t, letter, "left")
            print(line, "".join(t))
        elif line.startswith("rotate"):
            d, n = rotatenum.search(line).groups()
            e = "right" if d == "left" else "left"
            t = rotate(t, n, e)
            print(line, "".join(t))
        elif line.startswith("swap letter"):
            a, b = swap_ltr.search(line).groups()
            t = swapltr(t, a, b)
            print(line, "".join(t))
        elif line.startswith("swap position"):
            x, y = swap_pos.search(line).groups()
            t = swappos(t, x, y)
            print(line, "".join(t))
        elif line.startswith("reverse positions"):
            x, y = rev_pos.search(line).groups()
            t = reverse_s(t, x, y)
            print(line, "".join(t))
        elif line.startswith("move position"):
            x, y = move_pos.search(line).groups()
            t = move(t, y, x)
            print(line, "".join(t))

    print("Part One: %s " % ("".join(s)))
    print("Part Two: %s " % ("".join(t)))


if __name__ == "__main__":
    main()

    # print('a_______', ''.join(rotate_on_ltr([x for x in 'a_______'], 'a')))
    # print('_a______', ''.join(rotate_on_ltr([x for x in '_a______'], 'a')))
    # print('__a_____', ''.join(rotate_on_ltr([x for x in '__a_____'], 'a')))
    # print('___a____', ''.join(rotate_on_ltr([x for x in '___a____'], 'a')))
    # print('____a___', ''.join(rotate_on_ltr([x for x in '____a___'], 'a')))
    # print('_____a__', ''.join(rotate_on_ltr([x for x in '_____a__'], 'a')))
    # print('______a_', ''.join(rotate_on_ltr([x for x in '______a_'], 'a')))
    # print('_______a', ''.join(rotate_on_ltr([x for x in '_______a'], 'a')))

# fehcgadb
