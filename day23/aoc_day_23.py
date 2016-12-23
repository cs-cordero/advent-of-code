#!/bin/python3

import math

def day23(lines):
    registers = {'a': 7, 'b': 0, 'c': 0, 'd': 0}
    i = 0
    k = 0
    a = 0
    while i < len(lines):
        if k > 0:
            i += 1
            k -= 1
            continue
        line = lines[i]

        try:
            x = registers[line[1]]
        except KeyError:
            x = int(line[1])

        if line[0] == 'cpy':
            try:
                registers[line[2]] = registers.get(line[1], x)
            except:
                pass
        elif line[0] == 'inc':
            registers[line[1]] += 1
        elif line[0] == 'dec':
            registers[line[1]] -= 1
        elif line[0] == 'jnz' and x != 0:
            try:
                jump = int(line[2])
            except:
                jump = int(registers[line[2]])
            if jump < 0:
                i = max(i + jump, 0)
                continue
            else:
                k += jump
                continue
        elif line[0] == 'tgl':
            m = i + registers[line[1]]
            if m >= len(lines):
                i += 1
                continue
            tgl_line = lines[m]
            if tgl_line[0] in ('inc' 'dec' 'tgl'):
                tgl_line[0] = 'inc' if tgl_line[0] != 'inc' else 'dec'
            elif tgl_line[0] in ('jnz' 'cpy'):
                tgl_line[0] = 'cpy' if tgl_line[0] == 'jnz' else 'jnz'
        i += 1
    return registers['a']


with open('aoc_day23_input.txt') as f:
    r = [line.strip('\n').split(' ') for line in f.readlines()]


print('Part One: %d' % day23(r))
print('Part Two: %d' % (math.factorial(12) + 5112))