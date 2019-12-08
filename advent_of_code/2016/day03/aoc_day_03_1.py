#!/bin/python3


def squares_with_three_sides(sides):
    perimeter = sum(sides)

    for side in sides:
        if side >= perimeter - side:
            return False

    return True


if __name__ == "__main__":
    counter = 0
    with open("aoc_day_03_input.txt", "r") as f:
        for line in f:
            processed_line = list(map(int, filter(None, line.strip().split(" "))))
            counter += squares_with_three_sides(processed_line)
    print(counter)
