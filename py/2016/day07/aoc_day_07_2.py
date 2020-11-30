#!/usr/bin/python3

import re


def find_all_aba(line):
    aba = []
    for i in range(1, len(line) - 1):
        if (
            line[i - 1 : i + 2] == "".join(reversed(line[i - 1 : i + 2]))
            and line[i] != line[i - 1]
        ):
            aba.append(line[i - 1 : i + 2])
    return aba


def internet_protocol_v7(IP):
    supernets = re.split(r",", re.sub(r"\[\w+\]", r",", IP))
    hypernets = re.findall(r"\[(\w+)\]", IP)

    all_aba = []
    for supernet in supernets:
        all_aba.extend(find_all_aba(supernet))

    for aba in all_aba:
        # invert the aba
        bab = aba[1] + aba[0] + aba[1]
        for hypernet in hypernets:
            if bab in hypernet:
                return True

    return False


def main():
    print("Sample 1: %s" % internet_protocol_v7("aba[bab]xyz"))
    print("Sample 2: %s" % internet_protocol_v7("xyx[xyx]xyx"))
    print("Sample 3: %s" % internet_protocol_v7("aaa[kek]eke"))
    print("Sample 4: %s" % internet_protocol_v7("zazbz[bzb]cdb"))
    with open("aoc_day_07_input.txt", "r") as f:
        counter = 0
        for line in f:
            if internet_protocol_v7(line.strip()):
                counter += 1
    return counter


if __name__ == "__main__":
    print("Challenge: %s" % main())
