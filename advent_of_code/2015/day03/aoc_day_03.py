#!/bin/python3


def aoc(instructions):
    santa_location = [0, 0]
    santa_visited = set([tuple(santa_location)])

    santa_w_robot_location = [0, 0]
    robot_location = [0, 0]
    robot_visited = set([tuple(santa_w_robot_location)])
    is_santas_move = True

    instruction_map = {
        '^': [0, 1],
        '>': [1, 0],
        'v': [0,-1],
        '<': [-1,0]
    }

    def move_santa(location, delta):
        location[0] += delta[0]
        location[1] += delta[1]

    for instruction in instructions:
        delta = instruction_map.get(instruction, [0, 0])

        move_santa(santa_location, delta)
        santa_visited.add(tuple(santa_location))
        
        loc = santa_w_robot_location if is_santas_move else robot_location
        move_santa(loc, delta)
        robot_visited.add(tuple(loc))
        is_santas_move = not is_santas_move
    
    print('Part 1:  {}.'.format(len(santa_visited)))
    print('Part 2:  {}.'.format(len(robot_visited)))


if __name__ == '__main__':
    with open('aoc_day_03_input.txt') as f:
        instructions = ''.join(f.read().splitlines())
    aoc(instructions)
