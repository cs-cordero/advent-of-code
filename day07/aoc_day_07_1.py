#!/usr/bin/python3

import re


def find_abba(line):
    for i in range(len(line)-2):
        if (line[i-1:i+1] == ''.join(reversed(line[i+1:i+3])) and
           line[i] != line[i-1]):
            return True
    return False


def internet_protocol_v7(IP):
    supernets = re.split(r',', re.sub(r'\[\w+\]', r',', IP))
    hypernets = re.findall(r'\[(\w+)\]', IP)

    outside = False
    inside = False

    for supernet in supernets:
        if find_abba(supernet):
            outside = True
            break

    for hypernet in hypernets:
        if find_abba(hypernet):
            inside = True
            break

    return outside and not inside


def main():
    print('Sample 1: %s' % internet_protocol_v7('abba[mnop]qrst'))
    print('Sample 2: %s' % internet_protocol_v7('abcd[bddb]xyyx'))
    print('Sample 3: %s' % internet_protocol_v7('aaaa[qwer]tyui'))
    print('Sample 4: %s' % internet_protocol_v7('ioxxoj[asdfgh]zxcvbn'))
    with open('aoc_day_07_input.txt', 'r') as f:
        counter = 0
        for line in f:
            if internet_protocol_v7(line.strip()):
                counter += 1
    return counter


if __name__ == '__main__':
    print('Challenge: %s' % main())
