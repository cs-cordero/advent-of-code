#!/bin/python3

import re
# SCRAMBLE = 'abcdefgh'
# SCRAMBLE2 = 'fbgdceah'
# SCRAMBLE2 = 'gbhafcde'
SCRAMBLE = 'abcde'
SCRAMBLE2 = 'decab'
S_LEN = len(SCRAMBLE)

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


def rotate(s, steps, direction='right'):
    steps = int(steps)
    steps *= -1 if direction == 'right' else 1
    s = s[steps:] + s[:steps]
    return s


def rotate_on_ltr(s, x, rev='right'):
    for i in range(S_LEN):
        if s[i] == x:
            steps = i
            break
    steps += 2 if steps >= 4 else 1
    steps %= S_LEN
    return rotate(s, steps, rev)


def move(s, x, y):
    x, y = map(int, (x, y))
    s.insert(y, s.pop(x))
    return s


def reverse_s(s, x, y):
    x, y = map(int, (x, y))
    s = s[:x] + list(reversed(s[x:y+1])) + s[y+1:]
    return s


def main():
    s = [a for a in SCRAMBLE]
    t = [a for a in SCRAMBLE2]

    rotateltr = re.compile(r'letter (\w)')
    rotatenum = re.compile(r'rotate (left|right) (\d)')
    swap_ltr = re.compile(r'letter (\w) with letter (\w)')
    swap_pos = re.compile(r'position (\w) with position (\w)')
    rev_pos = re.compile(r'(\d) through (\d)')
    move_pos = re.compile(r'position (\w) to position (\w)')

    with open('aoc_day21_sample.txt') as f:
        inst_stack = f.readlines()

    for line in inst_stack:
        if line.startswith('rotate based'):
            letter = rotateltr.search(line).group(1)
            s = rotate_on_ltr(s, letter)
        elif line.startswith('rotate'):
            d, n = rotatenum.search(line).groups()
            s = rotate(s, n, d)
        elif line.startswith('swap letter'):
            a, b = swap_ltr.search(line).groups()
            s = swapltr(s, a, b)
        elif line.startswith('swap position'):
            x, y = swap_pos.search(line).groups()
            s = swappos(s, x, y)
        elif line.startswith('reverse positions'):
            x, y = rev_pos.search(line).groups()
            s = reverse_s(s, x, y)
        elif line.startswith('move position'):
            x, y = move_pos.search(line).groups()
            s = move(s, x, y)

    while inst_stack:
        line = inst_stack.pop()
        if line.startswith('rotate based'):
            letter = rotateltr.search(line).group(1)
            t = rotate_on_ltr(t, letter, 'left')
            print(''.join(t))
        elif line.startswith('rotate'):
            d, n = rotatenum.search(line).groups()
            e = 'right' if d == 'left' else 'left'
            t = rotate(t, n, e)
            print(''.join(t))
        elif line.startswith('swap letter'):
            a, b = swap_ltr.search(line).groups()
            t = swapltr(t, a, b)
            print(''.join(t))
        elif line.startswith('swap position'):
            x, y = swap_pos.search(line).groups()
            t = swappos(t, x, y)
            print(''.join(t))
        elif line.startswith('reverse positions'):
            x, y = rev_pos.search(line).groups()
            t = reverse_s(t, x, y)
            print(''.join(t))
        elif line.startswith('move position'):
            x, y = move_pos.search(line).groups()
            t = move(t, y, x)
            print(''.join(t))


    print('Part One: %s ' % (''.join(s)))
    print('Part Two: %s ' % (''.join(t)))

if __name__ == '__main__':
    main()

# fehcgadb