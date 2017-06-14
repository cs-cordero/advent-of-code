#!/bin/python3

# Advent of Code Day 1, Part 1

def easter_bunny_headquarters(directions):
    # Array indicating number of times moved in N E S W order;
    movements = [0, 0, 0, 0]
    current_direction = 0

    for direction in directions:
        current_direction += -1 if direction[0] == "L" else 1
        current_direction %= 4
        movements[current_direction] += int(direction[1:])

    return abs(movements[0] - movements[2]) + abs(movements[1] - movements[3])


sample_1 = "R2, L3"
sample_2 = "R2, R2, R2"
sample_3 = "R5, L5, R5, R3"
print("Sample 1: {}".format(easter_bunny_headquarters(sample_1.split(', '))))
print("Sample 2: {}".format(easter_bunny_headquarters(sample_2.split(', '))))
print("Sample 3: {}".format(easter_bunny_headquarters(sample_3.split(', '))))

advent_input = "L1, L3, L5, L3, R1, L4, L5, R1, R3, L5, R1, L3, L2, L3, R2, R2, L3, L3, R1, L2, R1, L3, L2, R4, R2, L5, R4, L5, R4, L2, R3, L2, R4, R1, L5, L4, R1, L2, R3, R1, R2, L4, R1, L2, R3, L2, L3, R5, L192, R4, L5, R4, L1, R4, L4, R2, L5, R45, L2, L5, R4, R5, L3, R5, R77, R2, R5, L5, R1, R4, L4, L4, R2, L4, L1, R191, R1, L1, L2, L2, L4, L3, R1, L3, R1, R5, R3, L1, L4, L2, L3, L1, L1, R5, L4, R1, L3, R1, L2, R1, R4, R5, L4, L2, R4, R5, L1, L2, R3, L4, R2, R2, R3, L2, L3, L5, R3, R1, L4, L3, R4, R2, R2, R2, R1, L4, R4, R1, R2, R1, L2, L2, R4, L1, L2, R3, L3, L5, L4, R4, L3, L1, L5, L3, L5, R5, L5, L4, L2, R1, L2, L4, L2, L4, L1, R4, R4, R5, R1, L4, R2, L4, L2, L4, R2, L4, L1, L2, R1, R4, R3, R2, R2, R5, L1, L2"
print("Challenge: {}".format(easter_bunny_headquarters(advent_input.split(', '))))