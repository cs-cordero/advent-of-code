#!/bin/python3


def assembly_to_normal(a):
    constant = 282 * 9
    a += constant

    solution = []
    while a > 0:
        solution.append(a % 2)
        a //= 2

    solution = ''.join(map(str, solution))
    return not (solution.count('11') or solution.count('00')) \
               and solution[0] == '0'


def main():
    a = 1
    while True:
        if assembly_to_normal(a):
            return a
        a += 1


if __name__ == '__main__':
    print('Day 25: %d' % main())