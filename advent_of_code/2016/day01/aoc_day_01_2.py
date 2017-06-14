#!/bin/python3

# Advent of Code Day 1, Part 2

def easter_bunny_headquarters(directions):
    # Array indicating number of times moved in N E S W order;
    current_location = [0, 0]
    current_direction = 0
    visited_set = set()
    visited_set.add(tuple(current_location))

    for direction in directions:
        current_direction += -1 if direction[0] == "L" else 1
        current_direction %= 4

        pos_change_i = 1 if current_direction == 0 or current_direction == 2 else 0
        dlt_change_i = -1 if current_direction == 2 or current_direction == 3 else 1

        for i in range(int(direction[1:])):
            current_location[pos_change_i] += dlt_change_i
            if tuple(current_location) in visited_set:
                break
            visited_set.add(tuple(current_location))
        else:
            continue
        break


    return current_location, abs(current_location[0] + current_location[1])


sample_1 = "R8, R4, R4, R8"
print("Sample 1: {}".format(easter_bunny_headquarters(sample_1.split(', '))))

advent_input = "L1, L3, L5, L3, R1, L4, L5, R1, R3, L5, R1, L3, L2, L3, R2, R2, L3, L3, R1, L2, R1, L3, L2, R4, R2, L5, R4, L5, R4, L2, R3, L2, R4, R1, L5, L4, R1, L2, R3, R1, R2, L4, R1, L2, R3, L2, L3, R5, L192, R4, L5, R4, L1, R4, L4, R2, L5, R45, L2, L5, R4, R5, L3, R5, R77, R2, R5, L5, R1, R4, L4, L4, R2, L4, L1, R191, R1, L1, L2, L2, L4, L3, R1, L3, R1, R5, R3, L1, L4, L2, L3, L1, L1, R5, L4, R1, L3, R1, L2, R1, R4, R5, L4, L2, R4, R5, L1, L2, R3, L4, R2, R2, R3, L2, L3, L5, R3, R1, L4, L3, R4, R2, R2, R2, R1, L4, R4, R1, R2, R1, L2, L2, R4, L1, L2, R3, L3, L5, L4, R4, L3, L1, L5, L3, L5, R5, L5, L4, L2, R1, L2, L4, L2, L4, L1, R4, R4, R5, R1, L4, R2, L4, L2, L4, R2, L4, L1, L2, R1, R4, R3, R2, R2, R5, L1, L2"
print("Challenge: {}".format(easter_bunny_headquarters(advent_input.split(', '))))