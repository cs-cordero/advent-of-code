from collections import deque
from typing import NamedTuple


TEST_DATA = """
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
"""

def read_file(filepath, use_test_data = False):
    if not use_test_data:
        with open(filepath) as f:
            for line in f.readlines():
                yield line.strip()
    else:
        for line in TEST_DATA.strip().split('\n'):
            yield line.strip()


class Point(NamedTuple):
    x: int
    y: int

    def __add__(self, other):
        x1, y1 = self
        x2, y2 = other
        return Point(x1 + x2, y1 + y2)


def solution():
    use_test = False
    grid = {Point(500, 0): '+'}
    grid = {}
    for line in read_file('input.txt', use_test):
        left, right = line.split(', ')
        left_coordinate, left_value = left.split('=')
        right_coordinate, right_range = right.split('=')
        right_start, right_end = map(int, right_range.split('..'))

        for right_value in range(right_start, right_end+1):
            coordinate = {
                left_coordinate: int(left_value),
                right_coordinate: right_value,
            }
            point = Point(**coordinate)
            grid[point] = '#'

    all_y = [y for x, y in grid.keys()]
    initial_miny, initial_maxy = min(all_y), max(all_y)
    grid[Point(500, 0)] = '+'

    queue = deque([Point(500, 1)])
    fill_queue = deque()
    while queue or fill_queue:
        if not fill_queue:
            # Falling Phase
            point = queue.popleft()
            if point.y > initial_maxy or point.y < 0:
                continue
            # print(point.y)

            below_point = point + Point(0, 1)
            if point not in grid and grid.get(point+(0,-1)) in ('|', '+'):
                grid[point] = '|'
                if below_point not in grid:
                    queue.append(below_point)
                    continue

            # if point == (501, 39):
            #     import pdb; pdb.set_trace()
            if grid[below_point] == '#':
                fill_queue.append(point)
            elif grid[below_point] == '~':
                left_pointer = point + (-1, 0)
                while grid.get(left_pointer) not in ('|', '#'):
                    lower_left_pointer = left_pointer + (0, 1)
                    if grid.get(lower_left_pointer) in (None, '|'):
                        break
                    left_pointer = left_pointer + (-1, 0)
                right_pointer = point + (1, 0)
                while grid.get(right_pointer) not in ('|', '#'):
                    lower_right_pointer = right_pointer + (0, 1)
                    if grid.get(lower_right_pointer) in (None, '|'):
                        break
                    right_pointer = right_pointer + (1, 0)
                if grid.get(left_pointer) == '#' and grid.get(right_pointer) == '#':
                    fill_queue.append(point)
            continue

        else:
            starting_fill_point = fill_queue[0]
            while fill_queue:
                fill_point = fill_queue.popleft()
                fill_value = grid.get(fill_point)
                below_fill_point = fill_point + (0, 1)
                below_fill_value = grid.get(below_fill_point)
                if fill_value in ('#', '~'):
                    continue
                if below_fill_value is None:
                    grid[fill_point] = '|'
                    queue.append(below_fill_point)
                    continue
                elif below_fill_value == '|':
                    assert False

                grid[fill_point] = '~'
                if fill_point in queue:
                    queue.remove(fill_point)
                    queue.append(fill_point + (0, -1))
                fill_queue.extend([
                    fill_point + (-1, 0),
                    fill_point + (1, 0),
                ])
            left_pointer = starting_fill_point
            while grid.get(left_pointer) not in (None, '|', '#'):
                left_pointer = left_pointer + (-1, 0)
            right_pointer = starting_fill_point
            while grid.get(right_pointer) not in (None, '|', '#'):
                right_pointer = right_pointer + (1, 0)
            if grid.get(left_pointer) == '#' and grid.get(right_pointer) == '#':
                fill_queue.append(starting_fill_point + (0, -1))
    return (
        grid,
        sum(1 for x, y in grid.items() if y in ('|', '~')
            if x.y >= initial_miny and x.y <= initial_maxy),
    )


def solution2(grid):
    for point, value in grid.items():
        if value == '|':
            grid[point] = '.'

            left_pointer = point + (-1, 0)
            while grid.get(left_pointer) not in (None, '|', '#'):
                left_pointer = left_pointer + (-1, 0)
            right_pointer = point + (1, 0)
            while grid.get(right_pointer) not in (None, '|', '#'):
                right_pointer = right_pointer + (1, 0)

            left = grid.get(left_pointer)
            right = grid.get(right_pointer)
            if left is None and right is None:
                continue
            elif left is None and right in ('#', '|'):
                for col in range(point.x, right_pointer.x):
                    grid[Point(col, point.y)] = '.'
            elif left in ('#', '|') and right is None:
                for col in range(left_pointer.x+1, point.x+1):
                    grid[Point(col, point.y)] = '.'
            elif left == '|' and right == '|':
                a = point.y if abs(left_pointer.x - point.x) <= 1 else left_pointer.x
                b = point.y if abs(right_pointer.x - point.x) <= 1 else right_pointer.x
                for col in range(a, b+1):
                    grid[Point(col, point.y)] = '.'

    return (
        grid,
        sum(1 for x, y in grid.items() if y in ('~')
            if x.y >= 5 and x.y <= 1719),
    )


def display_grid(grid, y_start=None, y_end=None, x_start=None, x_end=None):
    all_x = [x for x, y in grid.keys()]
    all_y = [y for x, y in grid.keys()]
    minx, maxx = x_start or min(all_x), x_end or max(all_x)
    miny, maxy = y_start or min(all_y), y_end or max(all_y)

    for row in range(miny, maxy+1):
        _row = ''
        for col in range(minx, maxx+1):
            _row += grid.get(Point(col, row), '.')
        print(_row)


grid, part1 = solution()
print(part1)

grid, part2 = solution2(grid)
print(part2)
