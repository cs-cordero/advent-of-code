#!/bin/python3
# -*- coding: utf-8 -*-

import collections
import copy
import re


def floor_isValid(floors, floor):
    current_floor = floors[floor]
    # print('current_floor: {}'.format(current_floor))
    generators = [gx[0] for gx in current_floor if gx[1] == "generator"]
    # print('generators: {}'.format(generators))
    if not generators:
        return True

    for item in current_floor:
        if item[1] == "microchip" and item[0] in generators:
            continue
        elif item[1] == "microchip":
            return False

    return True


class memoized(object):
    def __init__(self, func):
        self.func = func
        self.cache = {1: {}, 2: {}, 3: {}, 4: {}}

    def __call__(self, *args):
        fs = frozenset(args[0][args[1]])
        if not isinstance(fs, collections.Hashable):
            return False
        if fs in self.cache[args[1]]:
            # print('Already memoized.')
            return False
        else:
            self.cache[args[1]][fs] = False

        return self.func(*args)


def get_combinations(floors, floor):
    current_floor = floors[floor]
    combinations = []
    for i in range(len(current_floor) - 1):
        for j in range(i + 1, len(current_floor)):
            combinations.append([current_floor[i], current_floor[j]])

    combinations.extend([[x] for x in current_floor])
    return combinations


@memoized
def solve(floors, floor, steps):
    # base cases:
    # if in memo, return
    # else memoize
    # if not a valid floor, return
    # else if complete, then return true

    # copy the floorplan for restoration
    # loop over possible moves
    # recurse backwards
    # restore the floors
    # recurse forwards
    # restore the floors
    # return false

    # if memoized, the decorator will handle that for us.
    if not floor_isValid(floors, floor):
        return False
    if floor == 4 and not floors[1] and not floors[2] and not floors[3]:
        return steps

    temp_floorplan = copy.deepcopy(floors)
    print("current floor %d: %s" % (floor, [" ".join(x) for x in floors[floor]]))
    combinations = get_combinations(floors, floor)
    if floor > 1 and len(floors[floor - 1]) > 0:
        for moved_items in combinations:
            if len(moved_items) > 1:
                continue
            for moved_item in moved_items:
                print(
                    "moving: %s, %d <-- %d" % (" ".join(moved_item), floor - 1, floor)
                )
                floors[floor - 1].append(moved_item)
                floors[floor].remove(moved_item)

            sol = solve(floors, floor - 1, steps + 1)
            if sol:
                return sol
            floors = copy.deepcopy(temp_floorplan)

    if floor < 4:
        for moved_items in combinations:
            print("New step {}:".format(steps))
            for moved_item in moved_items:
                print(
                    "moving: %s, %d --> %d" % (" ".join(moved_item), floor, floor + 1)
                )
                floors[floor + 1].append(moved_item)
                floors[floor].remove(moved_item)

            sol = solve(floors, floor + 1, steps + 1)
            if sol:
                return sol
            floors = copy.deepcopy(temp_floorplan)

    return False


def load_inputs(version="input"):
    floors = {}
    with open("aoc_day_11_{}.txt".format(version)) as f:
        i = 1
        for line in f:
            floors[i] = re.findall(
                r"(\w+)\s*(?:-compatible )*(microchip|generator)", line.strip()
            )
            i += 1
    return floors


floors = load_inputs("input2")
print(solve(floors, 1, 0))
