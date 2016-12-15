#!/bin/python3


# ['TG', 'TM', 'PG', 'PM', 'SG', 'SM', 'XG', 'XM', 'RG', 'RM']
# FLOORS = [
#     ['TG', 'TM', 'PG',  '.', 'SG',  '.',  '.',  '.',  '.',  '.'],
#     [ '.',  '.',  '.', 'PM',  '.', 'SM',  '.',  '.',  '.',  '.'],
#     [ '.',  '.',  '.',  '.',  '.',  '.', 'XG', 'XM', 'RG', 'RM'],
#     [ '.',  '.',  '.',  '.',  '.',  '.',  '.',  '.',  '.',  '.']
# ]

# FLOORS = [
#     ['TG', 'TM', 'PG', 'SG'],
#     ['PM', 'SM'],
#     ['XG', 'XM', 'RG', 'RM'],
#     []
# ]

FLOORS = [
    [],
    ['XG', 'XM', 'RG', 'RM', 'PG', 'PM', 'SG', 'SM'],
    [],
    []
]

def check_valid_floor(floor):
    current_floor = FLOORS[floor]
    generator_present = False
    for generator in ('TG', 'PG', 'SG', 'XG', 'RG'):
        if generator in current_floor:
            generator_present = True
            break

    for generator, microchip in (('TG', 'TM'),
                                 ('PG','PM'),
                                 ('SG','SM'),
                                 ('XG','XM'),
                                 ('RG', 'RM')):
        if (generator_present and \
            microchip in current_floor) and \
            not (generator in current_floor):
            return False
    return True


def get_combinations(floor):
    current_floor = FLOORS[floor]
    combinations = []
    for i in range(len(current_floor)-1):
        for j in range(i+1, len(current_floor)):
            combinations.append((current_floor[i], current_floor[j]))
    combinations.extend([(x,) for x in current_floor])
    return combinations


def solve(floor, steps):
    current_floor = FLOORS[floor]
    temp_curr = list(current_floor)
    valid = check_valid_floor(floor)
    if valid and floor == 3:
        return steps
    elif not valid and floor < 3:
        return False

    if len(FLOORS[1]) == 0:
        return steps

    if floor > 0 and len(FLOORS[floor-1]) > 0:
        temp_prev = list(FLOORS[floor-1])
        for item in list(FLOORS[floor]):
            print("WTF")
            print(FLOORS[floor], temp_curr, list(temp_curr))
            print(item, FLOORS[floor])
            FLOORS[floor-1].append(item)
            FLOORS[floor].remove(item)
            print('Moved %s to floor %d from floor %d' % (item, floor, floor-1))

            print('current_floor %d: %s' % (floor, FLOORS[floor]))
            print('prev_floor %d: %s' % (floor-1, FLOORS[floor-1]))
            if check_valid_floor(floor):
                solve(floor-1, steps+1)  # recurse
            # restore floors
            print('restoring...')
            print('%d: %s --> %s' % (floor, FLOORS[floor], list(temp_curr)))
            print('%d: %s --> %s' % (floor-1, FLOORS[floor-1], list(temp_prev)))
            FLOORS[floor-1] = list(temp_prev)
            FLOORS[floor] = list(temp_curr)
            print('completed restoring')


    if floor < 3:
        next_floor = FLOORS[floor+1]
        temp_next = list(next_floor)
        print('current_floor %d: %s' % (floor, FLOORS[floor]))
        for combo in get_combinations(floor):
            # print('removing %s from %s' % (combo, FLOORS[floor]))
            if len(FLOORS[floor]) > 1 and len(combo) < 2: continue
            for item in combo:
                FLOORS[floor+1].append(item)
                FLOORS[floor].remove(item)
                print('Moved %s to floor %d from floor %d' % (item, floor, floor+1))

            print('current_floor %d: %s' % (floor, FLOORS[floor]))
            print('next_floor %d: %s' % (floor+1, FLOORS[floor+1]))
            if check_valid_floor(floor):
                solve(floor+1, steps+1) # recurse
            # restore floors
            print('restoring...')
            print('%s --> %s' % (FLOORS[floor], list(temp_curr)))
            print('%s --> %s' % (FLOORS[floor+1], list(temp_next)))
            FLOORS[floor] = list(temp_curr)
            FLOORS[floor+1] = list(temp_next)
            print('completed restoring')

print(solve(1, 0))
# print(get_combinations(0))
# print(check_valid_floor(0))