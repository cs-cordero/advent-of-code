#!/bin/python3


def aoc(boxes):
    wrapping_paper = 0
    ribbon = 0

    for box in boxes:
        length, width, height = (int(dim) for dim in box)

        wrapping_paper += get_wrapping_paper_area(length, width, height)
        ribbon += get_ribbon_length(length, width, height)

    print("Part 1:  Wrapping Paper Size:  {}.".format(wrapping_paper))
    print("Part 2:  Ribbon Length:  {}.".format(ribbon))


def get_wrapping_paper_area(length, width, height):
    areas = (length * width, length * height, width * height)
    return 2 * sum(areas) + min(areas)


def get_ribbon_length(length, width, height):
    perimeters = (length + width, length + height, width + height)
    return 2 * min(perimeters) + (length * width * height)


if __name__ == "__main__":
    with open("aoc_day_02_input.txt") as f:
        instructions = [line.split("x") for line in f.readlines()]
    aoc(instructions)
