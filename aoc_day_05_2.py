#!/usr/bin/python3

import hashlib

def find_code(door_id):
    s = [None] * 8
    i = 0
    while None in s:
        m = hashlib.md5(door_id + str(i).encode('utf-8')).hexdigest()
        if m.startswith('00000'):
            print("{}: {}".format(door_id + str(i).encode('utf-8'), m))
            location = int(m[5], 16)
            if location < 8 and s[location] is None:
                s[location] = m[6]
        i += 1
    return ''.join(s)

sample = 'abc'.encode('utf-8')
print('Sample: {}'.format(find_code(sample)))

print('\n')

door_id = 'wtnhxymk'.encode('utf-8')
print('Challenge: {}'.format(find_code(door_id)))