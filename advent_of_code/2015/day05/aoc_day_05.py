#!/bin/python3


def aoc(strings):
    def check_string(s):
        vowel_count = 0
        repeat = False
        vowels = {'a', 'e', 'i', 'o', 'u'}
        badstr = {'ab', 'cd', 'pq', 'xy'}
        for i in range(len(s)):
            if s[i:i+2] in badstr:
                return False
            if not repeat and i < len(s) - 1 and s[i] == s[i+1]:
                repeat = True
            if vowel_count < 3 and s[i] in vowels:
                vowel_count += 1
        if repeat and vowel_count >= 3:
            return True
        return False

    def check_string_again(s):
        triplet = False
        repeat = False
        for i in range(len(s)-2):
            double = s[i:i+2]
            triple = s[i:i+3]
            if s[i+2:].count(double) > 0:
                repeat = True
            if triple[0] == triple[2]:
                triplet = True
        return triplet and repeat

    count = 0
    recount = 0
    for string in strings:
        count += 1 if check_string(string) else 0
        recount += 1 if check_string_again(string) else 0

    print('Part 1:  {}.'.format(count))
    print('Part 2:  {}.'.format(recount))


if __name__ == '__main__':
    with open('aoc_day_05_input.txt') as f:
        strings = f.read().splitlines()
    aoc(strings)
