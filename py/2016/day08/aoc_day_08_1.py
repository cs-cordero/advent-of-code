#!/bin/python3

import re


def rect(grid, a, b):
    for x in range(a):
        for y in range(b):
            grid[y][x] = "#"
    return grid


def rotate_row(grid, a, b):
    grid[a] = grid[a][-b:] + grid[a][:-b]
    return grid


def rotate_col(grid, a, b):
    column = []
    for i in range(len(grid)):
        column.append(grid[i][a])
    column = column[-b:] + column[:-b]
    for i in range(len(grid)):
        grid[i][a] = column.pop(0)
    return grid


def main(fn):
    grid = [["." for i in range(50)] for j in range(6)]
    with open(fn, "r") as f:
        for line in f:
            search_rect = re.search(r"rect (\d*)x(\d*)", line)
            search_rot8 = re.search(r"rotate\s(\w+)\s\w=(\d*)\sby\s(\d*)", line)
            if search_rect:
                rect(grid, int(search_rect.group(1)), int(search_rect.group(2)))

            elif search_rot8 and search_rot8.group(1) == "row":
                rotate_row(grid, int(search_rot8.group(2)), int(search_rot8.group(3)))

            elif search_rot8 and search_rot8.group(1) == "column":
                rotate_col(grid, int(search_rot8.group(2)), int(search_rot8.group(3)))

    counter = 0
    for row in grid:
        counter += row.count("#")
    return counter


if __name__ == "__main__":
    print(main("aoc_day_08_input.txt"))
