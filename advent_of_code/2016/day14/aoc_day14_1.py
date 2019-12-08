#!/bin/python3

import hashlib
import re

SALT = "ihaygndm"
SALT_sample = "abc"
KEY_STRETCH = 2016


def stretch_key(s, stretch_val):
    md5 = hashlib.md5(s).hexdigest()
    while stretch_val > 0:
        md5 = hashlib.md5(md5).hexdigest()
        stretch_val -= 1
    return md5


def day14(salt):
    triple = re.compile(r"(\w)\1{2}")
    quintuple = re.compile(r"(\w)\1{4}")

    keys = set()
    i = 0

    # Pulls extra keys to be on the safe side.
    while len(keys) < 75:
        a = stretch_key(salt + str(i), 0)
        a_m = quintuple.findall(a)
        for q in set(a_m):
            for j in range(max(i - 1000, 0), i):
                b = stretch_key(salt + str(j), 0)
                b_m = triple.search(b)
                if b_m and q == b_m.group(1):
                    keys.add(j)

                    print("%s from %d matched to %s on index %d" % (b, j, a, i))

        i += 1
    return sorted(keys)[63]


print("Sample: %d" % day14(SALT_sample))
print("Challenge: %d" % day14(SALT))
