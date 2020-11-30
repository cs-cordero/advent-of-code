#!/bin/python3

import hashlib


def aoc(key):
    i = 1
    leading_five = 0
    leading_six = 0
    while not leading_five or not leading_six:
        md5 = hashlib.md5("{}{}".format(key, str(i)).encode()).hexdigest()
        if not leading_five and int(md5[:5], 16) == 0:
            leading_five = i
        if not leading_six and int(md5[:6], 16) == 0:
            leading_six = i
        i += 1

    print("Part 1:  {}.".format(leading_five))
    print("Part 2:  {}.".format(leading_six))


if __name__ == "__main__":
    key = "iwrupvqb"
    aoc(key)
