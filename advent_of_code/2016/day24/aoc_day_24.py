#!/bin/python3

import collections
import copy
from itertools import permutations


def enqueue(grid, x, y, queue, steps):
    avoided_chars = {"#", "x"}
    x_max = len(grid)
    y_max = len(grid[0])
    if x - 1 >= 0 and grid[x - 1][y] not in avoided_chars:
        queue.append((x - 1, y, steps + 1))
    if x + 1 < x_max and grid[x + 1][y] not in avoided_chars:
        queue.append((x + 1, y, steps + 1))
    if y - 1 >= 0 and grid[x][y - 1] not in avoided_chars:
        queue.append((x, y - 1, steps + 1))
    if y + 1 < y_max and grid[x][y + 1] not in avoided_chars:
        queue.append((x, y + 1, steps + 1))
    return queue


def get_distances(grid, x, y):
    grid_copy = copy.deepcopy(grid)
    distances = {}

    queue = collections.deque()
    queue.append((x, y, 0))
    while queue:
        x, y, steps = queue.popleft()
        if grid[x][y] == "x":
            continue

        try:
            int(grid[x][y])
            distances.setdefault(grid[x][y], steps)
        except ValueError:
            pass

        grid[x][y] = "x"
        queue = enqueue(grid, x, y, queue, steps)

    grid = grid_copy
    return distances, grid


def main():
    grid = []
    targets = {}
    target_locs = {}
    with open("aoc_day_24_input.txt") as f:
        x = 0
        for line in f:
            line = line.strip()
            row_to_append = []
            for y in range(len(line)):
                try:
                    int(line[y])
                    target_locs.setdefault(line[y], (x, y))
                    targets.setdefault(line[y], dict())
                except ValueError:
                    pass
                row_to_append.append(line[y])

            grid.append(row_to_append)
            x += 1

    for key, val in target_locs.items():
        targets[key], grid = get_distances(grid, val[0], val[1])

    for target in targets:
        print(target, targets[target])

    part_one = []
    part_two = []
    perms = [list(perm) for perm in permutations(range(8), 8)]

    for perm in perms:
        new = sum(targets[str(perm[i - 1])][str(perm[i])] for i in range(1, len(perm)))
        part_one.append(new)

        if perm[0] == 0:
            if perm[-1]:
                perm.append(0)
            new = sum(
                targets[str(perm[i - 1])][str(perm[i])] for i in range(1, len(perm))
            )
            part_two.append(new)

    print("Part 1: %d" % min(part_one))
    print("Part 2: %d" % min(part_two))


if __name__ == "__main__":
    main()
