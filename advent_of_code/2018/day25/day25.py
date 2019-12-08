from typing import NamedTuple


class Point4D(NamedTuple):
    x: int
    y: int
    z: int
    t: int

    def distance_to(self, other: "Point4D") -> int:
        return sum(abs(a - b) for a, b in zip(self, other))


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.rstrip()


def solution():
    """ Brute force solution that takes a minute or two but who cares """
    points = [Point4D(*map(int, line.split(","))) for line in read_file("input.txt")]
    constellations = []

    while points:
        constellation = [points.pop()]
        i = 0
        while i < len(points):
            point = points[i]
            if any(point.distance_to(cpoint) <= 3 for cpoint in constellation):
                constellation.append(points.pop(i))
                i = 0
                continue
            i += 1
        constellations.append(constellation)
    return len(constellations)


print(solution())
