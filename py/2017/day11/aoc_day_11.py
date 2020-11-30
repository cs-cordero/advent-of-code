from collections import defaultdict


def solution(path):
    furthest_traveled_ever = 0
    coordinates = defaultdict(int)
    vector = {
        "n": (0, 1, -1),
        "ne": (1, 0, -1),
        "nw": (-1, 1, 0),
        "s": (0, -1, 1),
        "sw": (-1, 0, 1),
        "se": (1, -1, 0),
    }

    for step in path:
        differential = vector.get(step)
        coordinates["x"] += differential[0]
        coordinates["y"] += differential[1]
        coordinates["z"] += differential[2]
        distance_traveled = max(map(abs, coordinates.values()))
        furthest_traveled_ever = max(furthest_traveled_ever, distance_traveled)

    return distance_traveled, furthest_traveled_ever


if __name__ == "__main__":
    with open("aoc_day_11_input.txt", "r") as f:
        s = f.readlines()[0].strip().split(",")
    answer = solution(s)
    print("Part One: ", answer[0])
    print("Part Two: ", answer[1])
