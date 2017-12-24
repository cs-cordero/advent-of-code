from collections import defaultdict
from copy import deepcopy

class Vector:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __repr__(self):
        return str((self.x, self.y))

    def turn_left(self):
        if self.x == -1 and self.y == 0:
            self.x = 0
            self.y = -1
        elif self.x == 0 and self.y == -1:
            self.x = 1
            self.y = 0
        elif self.x == 1 and self.y == 0:
            self.x = 0
            self.y = 1
        elif self.x == 0 and self.y == 1:
            self.x = -1
            self.y = 0

    def turn_right(self):
        if self.x == -1 and self.y == 0:
            self.x = 0
            self.y = 1
        elif self.x == 0 and self.y == 1:
            self.x = 1
            self.y = 0
        elif self.x == 1 and self.y == 0:
            self.x = 0
            self.y = -1
        elif self.x == 0 and self.y == -1:
            self.x = -1
            self.y = 0

    @property
    def position(self):
        return self.x, self.y

def solution(infecteds, iterations, part_two=False):
    sporifica = Vector(0, 0)
    direction = Vector(-1, 0)
    counter = 0
    for _ in range(iterations):
        # sporifica turns
        if infecteds[sporifica.position] == 0:
            direction.turn_left()
        elif infecteds[sporifica.position] == 1:
            pass
        elif infecteds[sporifica.position] == 2:
            direction.turn_right()
        elif infecteds[sporifica.position] == 3:
            direction.turn_right()
            direction.turn_right()


        # sporifica cleans or infects
        if part_two:
            infecteds[sporifica.position] += 1
            infecteds[sporifica.position] %= 4
        else:
            status = infecteds[sporifica.position]
            infecteds[sporifica.position] = 0 if status == 2 else 2
        counter += 1 if infecteds[sporifica.position] == 2 else 0

        # sporifica moves forward
        sporifica += direction
    return counter

if __name__ == '__main__':
    with open('aoc_day_22_input.txt', 'r') as f:
        s = [[c for c in line.strip()] for line in f.readlines()]

    # s = [
    #     ['.', '.', '#'],
    #     ['#', '.', '.'],
    #     ['.', '.', '.']
    # ]

    infecteds = defaultdict(int)
    offset = len(s)//2
    for i in range(-(len(s)//2), len(s)//2+1):
        for j in range(-(len(s[0])//2), len(s[0])//2+1):
            if s[i+offset][j+offset] == '#':
                infecteds[(i, j)] = 2

    print(f'Part 1: {solution(deepcopy(infecteds), 10000)}')
    print(f'Part 2: {solution(deepcopy(infecteds), 10000000, True)}')
