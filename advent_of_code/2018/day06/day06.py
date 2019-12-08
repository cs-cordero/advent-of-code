from collections import defaultdict


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def get_closest_point_and_sum_of_distances(origin, all_points):
    absolute_closest = True
    distance = 1 << 31
    closest_point = None
    sum_of_distances = 0
    for i, point in all_points.items():
        next_distance = manhattan_distance(origin, point)
        if next_distance < distance:
            absolute_closest = True
            distance = next_distance
            closest_point = i
        elif next_distance == distance:
            absolute_closest = False
        sum_of_distances += next_distance

    if not absolute_closest:
        closest_point = None
    return (closest_point, sum_of_distances)


def manhattan_distance(point1, point2):
    x1, y1 = point1
    x2, y2 = point2
    return abs(x1 - x2) + abs(y1 - y2)


def get_grid_limits(points):
    x, y = zip(*points)
    return (min(x), max(x), min(y), max(y))


def solution():
    def parse_line(line):
        a, b = line.split(", ")
        return (int(a), int(b))

    all_points = {i: parse_line(line) for i, line in enumerate(read_file("input.txt"))}
    infinite_points = set()
    closest_counts = defaultdict(int)
    count_within_region = 0

    minx, maxx, miny, maxy = get_grid_limits(all_points.values())
    for x in range(minx, maxx + 1):
        for y in range(miny, maxy + 1):
            closest_point, sum_of_distances = get_closest_point_and_sum_of_distances(
                (x, y), all_points
            )
            if sum_of_distances < 10000:
                count_within_region += 1

            if closest_point is None:
                continue

            closest_counts[closest_point] += 1
            if x in (minx, maxx) or y in (miny, maxy):
                infinite_points.add(closest_point)

    for infinite_point in infinite_points:
        closest_counts.pop(infinite_point)

    return sorted(closest_counts.items(), key=lambda x: x[1]), count_within_region


print(solution())
