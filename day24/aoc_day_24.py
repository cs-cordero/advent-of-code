#!/bin/python3

import copy
import collections
import re


def enqueue(grid, x, y, queue, steps):
    avoided_chars = {'#', 'x'}
    x_max = len(grid)
    y_max = len(grid[0])
    if x - 1 >= 0 and grid[x-1][y] not in avoided_chars:
        queue.append((x-1, y, steps+1))
    if x + 1 < x_max and grid[x+1][y] not in avoided_chars:
        queue.append((x+1, y, steps+1))
    if y - 1 >= 0 and grid[x][y-1] not in avoided_chars:
        queue.append((x, y-1, steps+1))
    if y + 1 < y_max and grid[x][y+1] not in avoided_chars:
        queue.append((x, y+1, steps+1))
    return queue


def get_distances(grid, x, y):
    grid_copy = copy.deepcopy(grid)
    distances = {}

    queue = collections.deque()
    queue.append((x, y, 0))
    while queue:
        x, y, steps = queue.popleft()
        if grid[x][y] == 'x':
            continue

        try:
            int(grid[x][y])
            distances.setdefault(grid[x][y], steps)
        except:
            pass

        grid[x][y] = 'x'
        queue = enqueue(grid, x, y, queue, steps)

    grid = grid_copy
    return distances, grid


def find_min(d, visited):
    curr_key = None
    for key in d:
        if key in visited:
            continue
        if not curr_key or (curr_key and d[curr_key] > d[key]):
            curr_key = key
    return curr_key


def find_path(d, current_key, visited, steps = 0):
    visited = copy.deepcopy(visited)
    while len(visited) < len(d):
        step_count = 0
        min_dist = min(x[1] for x in d[current_key].items() \
                       if x[0] not in visited)
        candidates = [key[0] for key in d[current_key].items() \
                     if key[1] == min_dist]
        if len(candidates) > 1:
            dists = []
            for candidate in candidates:
                dists.append((candidate, find_path(d, candidate, visited, d[current_key][candidate])))
            print(dists)
            return steps + min(dist[1] for dist in dists)
        steps += d[current_key][candidates[0]]
        current_key = candidates[0]
        visited.add(candidates[0])
    return steps


def main():
    grid = []
    targets = {}
    target_locs = {}
    start = (None, None)
    with open('aoc_day_24_input.txt') as f:
    # with open('aoc_day_24_sample.txt') as f:
        x = 0
        for line in f:
            line = line.strip()
            row_to_append = []
            for y in range(len(line)):
                try:
                    int(line[y])
                    target_locs.setdefault(line[y], (x, y))
                    targets.setdefault(line[y], dict())
                except:
                    pass
                row_to_append.append(line[y])

            grid.append(row_to_append)
            x += 1

    for key, val in target_locs.items():
        targets[key], grid = get_distances(grid, val[0], val[1])

    
    for key, val in targets.items():
        print(key, targets[key])

    visited = set('0')
    print(find_path(targets, '0', visited))


if __name__ == '__main__':
    print(main())
