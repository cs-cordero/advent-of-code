class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        self.x += other.x
        self.y += other.y

    @property
    def position(self):
        return (self.x, self.y)

class Grid:
    def __init__(self, grid):
        self.grid = grid
        self.current = Point(*self._find_starting_point())
        self.direction = Point(1, 0)
        self.seen = []
        self.steps = 0
        self.valid_paths = ('|', '-', '+')

    def mark(self):
        x, y = self.current.position
        try:
            self.grid[x][y] = 'x'
        except:
            print(x, y)
            raise

    def get(self):
        x, y = self.current.position
        return self.grid[x][y]

    def get_new_direction(self):
        x, y = self.current.position
        U = (-1, 0)
        D = (1, 0)
        L = (0, -1)
        R = (0, 1)
        for direction in (U, D, L, R):
            row, col = x + direction[0], y + direction[1]
            try:
                if self.grid[row][col] in self.valid_paths:
                    self.direction = Point(direction[0], direction[1])
                    return
            except:
                continue
        raise Exception('Could not find next direction')

    def go(self):
        grid_value = self.get()
        while grid_value in self.valid_paths or grid_value.isalpha():
            grid_value = self.get()
            if grid_value.isalpha() and grid_value != 'x' and grid_value != ' ':
                self.seen.append(grid_value)
            if grid_value == '+':
                self.get_new_direction()
            self.mark()
            self.current + self.direction
            self.steps += 1
        return self.seen, self.steps -1

    def _find_starting_point(self):
        for i in range(len(self.grid[0])):
            if self.grid[0][i] == '|':
                return (0, i)

def solution(grid):
    grid = Grid(grid)
    return grid.go()

if __name__ == '__main__':
    with open('aoc_day_19_input.txt', 'r') as f:
        grid = [[sq for sq in line.rstrip()] for line in f.readlines()]
    answer = solution(grid)
    print(f'Part 1:', ''.join(answer[0]))
    print(f'Part 2: {answer[1]}')
