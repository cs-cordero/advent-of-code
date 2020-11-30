def solution(target):
    grid_size = 15
    grid = [[0] * grid_size for i in range(grid_size)]
    grid[grid_size // 2][grid_size // 2] = 1

    direction = (-1, 0)
    x = grid_size // 2
    y = grid_size // 2 + 1
    next_val = get_sum(grid, x, y)
    while next_val < target:
        grid[x][y] = next_val
        x += direction[0]
        y += direction[1]
        direction = get_next_direction(grid, x, y)
        next_val = get_sum(grid, x, y)
    return next_val


def get_next_direction(grid, x, y):
    above = grid[x - 1][y]
    below = grid[x + 1][y]
    left = grid[x][y - 1]
    right = grid[x][y + 1]

    if below and not left:
        return (0, -1)
    elif right and not below:
        return (1, 0)
    elif above and not right:
        return (0, 1)
    elif left and not above:
        return (-1, 0)


def get_sum(grid, x, y):
    return sum(grid[i][j] for i in range(x - 1, x + 2) for j in range(y - 1, y + 2))


print(solution(361527))
