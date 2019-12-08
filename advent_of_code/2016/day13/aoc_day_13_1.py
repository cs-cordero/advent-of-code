#!/bin/python3


FAVORITE_NUMBER = 1362
GRID = [[0 for j in range(50)] for i in range(50)]
TARGET = (31, 39)


def check_wall(coordinates):
    x, y = coordinates

    if x < 0 or y < 0 or x >= 50 or y >= 50:
        return False
    elif GRID[x][y] != 0:
        return False

    current = x * x + 3 * x + 2 * x * y + y + y * y
    current += FAVORITE_NUMBER
    current = bin(current)[2:].count("1")

    if current % 2 != 0:
        GRID[x][y] = -1
        return False
    else:
        return True


def day13():
    queue = [(1, 1, 0)]
    while queue:
        x, y, steps = queue.pop(0)
        if (x, y) == TARGET:
            return steps

        GRID[x][y] = steps
        if check_wall((x - 1, y)):
            queue.append((x - 1, y, steps + 1))
        if check_wall((x + 1, y)):
            queue.append((x + 1, y, steps + 1))
        if check_wall((x, y - 1)):
            queue.append((x, y - 1, steps + 1))
        if check_wall((x, y + 1)):
            queue.append((x, y + 1, steps + 1))

    return False


print(day13())
