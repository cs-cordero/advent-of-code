#!/bin/python3


def check_triangle(sides):
    perimeter = sum(sides)

    for side in sides:
        if side >= perimeter - side:
            return False

    return True


def squares_with_three_sides(file):
    counter = 0
    with open(file, "r") as f:
        while True:
            line1 = f.readline()
            line2 = f.readline()
            line3 = f.readline()
            if not line1 or not line2 or not line3:
                break

            processed_lines = [
                list(map(int, filter(None, line1.strip().split(" ")))),
                list(map(int, filter(None, line2.strip().split(" ")))),
                list(map(int, filter(None, line3.strip().split(" ")))),
            ]

            for col in range(3):
                triangle = [processed_lines[row][col] for row in range(3)]
                counter += check_triangle(triangle)

    return counter


if __name__ == "__main__":
    print(squares_with_three_sides("aoc_day_03_input.txt"))
