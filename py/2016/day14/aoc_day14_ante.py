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
    quadruple = re.compile(r"(\w)\1{3}")
    sentuple = re.compile(r"(\w)\1{6}")

    keys = set()
    i = 0

    # Pulls extra keys to be on the safe side.
    while len(keys) < 600:
        a = stretch_key(salt + str(i), 0)
        a_m = sentuple.findall(a)
        for q in set(a_m):
            for j in range(max(i - 100000, 0), i):
                b = stretch_key(salt + str(j), 0)
                b_m = quadruple.search(b)
                if b_m and q == b_m.group(1):
                    keys.add(j)

        i += 1
    return sorted(keys)[511]


print("Challenge: %d" % day14("yjdafjpo"))
