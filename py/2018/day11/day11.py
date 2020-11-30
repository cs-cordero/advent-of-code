PUZZLE_INPUT = 9424


def calculate_power_level(point, serial):
    px, py = point
    rack_id = px + 10
    power_level = rack_id * py
    power_level += serial
    power_level *= rack_id
    power_level = int(str(power_level)[-3])
    power_level -= 5
    return power_level


def get_grid_sums(grid, topleft):
    px, py = topleft
    assert px <= 298
    assert py <= 298
    px -= 1
    py -= 1

    mapping = {}
    points_in_grid = [(px + x, py + y) for x in range(3) for y in range(3)]
    current_sum = sum(grid[y][x] for x, y in points_in_grid)
    mapping[3] = current_sum

    for i in range(3, 300):
        rgt_edge = [(px + x, py + i) for x in range(i + 1)]
        btm_edge = [(px + i, py + y) for y in range(i + 1)][:-1]
        next_edge = rgt_edge + btm_edge

        try:
            current_sum += sum(grid[y][x] for x, y in next_edge)
        except IndexError:
            break
        mapping[i + 1] = current_sum
    return mapping


def solution(serial):
    size = 300
    grid = [
        [calculate_power_level((x, y), serial) for x in range(1, size + 1)]
        for y in range(1, size + 1)
    ]

    mapping = {}
    for x in range(1, size - 1):
        for y in range(1, size - 1):
            next_map = get_grid_sums(grid, (x, y))
            for grid_size, power in next_map.items():
                if grid_size not in mapping:
                    mapping[grid_size] = ((x, y), power)
                else:
                    current_point, current_power = mapping[grid_size]
                    if power > current_power:
                        mapping[grid_size] = ((x, y), power)

    part1 = mapping[3]
    part2 = sorted(mapping.items(), key=lambda x: x[1][1])[-1]
    return part1, part2


print(solution(PUZZLE_INPUT))
