#!/bin/python3

import re
import collections


def b_search(l, target, start, end):
    if start == end:
        return start

    search_ind = (start + end) // 2
    midpoint = l[search_ind][3]

    if target > midpoint:
        return b_search(l, target, search_ind + 1, end)
    else:
        return b_search(l, target, start, search_ind)


def get_movements(grid, x, y, queue):
    if x - 1 >= 0 and not grid[x-1][y] in ['X', 'G'] \
       and not isinstance(grid[x-1][y], int) and (x, y) not in queue:
        grid[x-1][y] = grid[x][y] + 1
        queue.append((x-1, y))
    if x + 1 < 34 and not grid[x+1][y] in ['X', 'G'] \
       and not isinstance(grid[x+1][y], int) and (x, y) not in queue:
        grid[x+1][y] = grid[x][y] + 1
        queue.append((x+1, y))
    if y - 1 >= 0 and not grid[x][y-1] in ['X', 'G'] \
       and not isinstance(grid[x][y-1], int) and (x, y) not in queue:
        grid[x][y-1] = grid[x][y] + 1
        queue.append((x, y-1))
    if y + 1 < 30 and not grid[x][y+1] in ['X', 'G'] \
       and not isinstance(grid[x][y+1], int) and (x, y) not in queue:
        grid[x][y+1] = grid[x][y] + 1
        queue.append((x, y+1))
    return queue


def bfs(grid, x, y, target, steps=0):
    queue = collections.deque()
    queue.append((x, y))
    # enqueue
    grid[x][y] = 0
    while queue:
        steps = grid[x][y]
        x, y = queue.popleft()
        if (x, y) == target:
            return steps
        get_movements(grid, x, y, queue)
    return False


def main():
    grid = [['.' for i in range(30)] for j in range(34)]
    grid[33][0] = 'G'
    coordinates = re.compile(r'node-x(\d+)-y(\d+)')
    with open('aoc_day_22_input.txt') as f:
        nodes = []
        for line in f.readlines()[2:]:
            new_line = [re.sub(r'T', r'', x) for x
                        in line.strip().split(' ') if x != '']
            new_line[1:4] = map(int, new_line[1:4])
            x, y = map(int, coordinates.search(new_line[0]).groups())
            if new_line[2] > 110:
                grid[x][y] = 'X'
            elif new_line[2] == 0:
                grid[x][y] = '_'
                curr_x = x
                curr_y = y
            nodes.append(new_line)
    nodes.sort(key=lambda x: x[3])

    counter = 0
    node_len = len(nodes)
    for i, node in enumerate(nodes):
        if node[2] == 0:
            continue
        j = b_search(nodes, node[2], 0, node_len)
        j += 1 if i == j else 0
        counter += max(node_len - j, 0)

    print('Part One: %d' % counter)

    steps = bfs(grid, curr_x, curr_y, (32, 0)) + (32 * 5) + 1
    print('Part Two: %d' % steps)


main()
