from collections import deque

with open("aoc_day_19_input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]

target = lines[-1]
operations = []
for line in lines[:-2]:
    operations.append(line.split(" => "))


def solution1():
    seen = set()
    for operation in operations:
        source, replacement = operation
        for i in range(len(target)):
            if target[i : i + len(source)] == source:
                seen.add(target[:i] + replacement + target[i + len(source) :])
    return seen


def solution2():
    molecule_count = len([capital for capital in target if capital == capital.upper()])
    molecule_rn = target.count("Rn")
    molecule_ar = target.count("Ar")
    molecule_y = target.count("Y")
    return molecule_count - molecule_rn - molecule_ar - 2 * molecule_y - 1
