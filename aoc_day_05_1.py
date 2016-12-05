#!/usr/bin/python3

import hashlib


def find_code(door_id):
    s = ''
    i = 0
    while len(s) < 8:
        m = hashlib.md5(door_id + str(i).encode('utf-8')).hexdigest()
        if m[:5] == '00000':
            print("{}: {}".format(door_id + str(i).encode('utf-8'), m))
            s += m[5]
        i += 1
    return s

sample = 'abc'.encode('utf-8')
print('Sample: {}'.format(find_code(sample)))
door_id = 'wtnhxymk'.encode('utf-8')
print('Challenge: {}'.format(find_code(door_id)))