#!/bin/python3


FAVORITE_NUMBER = 1362
GRID = [['.' for j in range(30)] for i in range(30)]


def check_wall(coordinates):
    x, y = coordinates
    
    if x < 0 or y < 0 or x >= 30 or y >= 30: return False
    elif GRID[x][y] != '.': return False

    current = x*x + 3*x + 2*x*y + y + y*y + FAVORITE_NUMBER
    current = bin(current)[2:].count('1') % 2 == 0
    if not current:
        GRID[x][y] = -1
        return False
    else:
        return True


def day13():
    queue = [(1, 1, 0)]
    while queue:
        x, y, steps = queue.pop(0)
        if steps > 50:
            continue

        GRID[x][y] = min(GRID[x][y], steps)

        for candidate in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
            if check_wall(candidate):
                queue.append((candidate[0], candidate[1], steps+1))

    
    return len([x for row in GRID for x in row if 0 <= x <= 50])


print(day13())