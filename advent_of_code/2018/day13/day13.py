from enum import Enum
from operator import attrgetter


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.rstrip()


class CartDirection(Enum):
    UP = (0, -1)
    DOWN = (0, 1)
    LEFT = (-1, 0)
    RIGHT = (1, 0)


CART_SHAPE = {
    '<': CartDirection.LEFT,
    '>': CartDirection.RIGHT,
    'v': CartDirection.DOWN,
    '^': CartDirection.UP,
}


class Cart:
    def __init__(self, id, position, direction):
        self.id = id
        self.position = position
        self.direction = direction
        self.turn_count = 0

    @property
    def x(self):
        return self.position[0]

    @property
    def y(self):
        return self.position[1]

    def turn(self):
        if self.turn_count == 0:
            self.direction = self._turn_left()
        elif self.turn_count == 1:
            pass
        elif self.turn_count == 2:
            self.direction = self._turn_right()
        self.turn_count += 1
        self.turn_count %= 3

    def _turn_left(self):
        if self.direction is CartDirection.LEFT:
            return CartDirection.DOWN
        elif self.direction is CartDirection.DOWN:
            return CartDirection.RIGHT
        elif self.direction is CartDirection.RIGHT:
            return CartDirection.UP
        elif self.direction is CartDirection.UP:
            return CartDirection.LEFT

    def _turn_right(self):
        if self.direction is CartDirection.LEFT:
            return CartDirection.UP
        elif self.direction is CartDirection.UP:
            return CartDirection.RIGHT
        elif self.direction is CartDirection.RIGHT:
            return CartDirection.DOWN
        elif self.direction is CartDirection.DOWN:
            return CartDirection.LEFT

    def tick(self, grid):
        px, py = self.position
        dx, dy = self.direction.value
        px += dx
        py += dy
        self.position = (px, py)
        if grid[py][px] == '\\':
            self.direction = (
                self._turn_right()
                if self.direction in (CartDirection.RIGHT, CartDirection.LEFT)
                else self._turn_left()
            )
        elif grid[py][px] == '/':
            self.direction = (
                self._turn_right()
                if self.direction in (CartDirection.UP, CartDirection.DOWN)
                else self._turn_left()
            )
        elif grid[py][px] == '+':
            self.turn()
        return self.position

    def __repr__(self):
        px, py = self.position
        return f'<Cart: ({str(px).rjust(3)}, {str(py).rjust(3)}), {self.direction.name}>'


def solution():
    grid = [line for line in read_file('input.txt')]
    carts = {}
    cart_positions = {}
    for row in range(len(grid)):
        for col, char in enumerate(grid[row]):
            cart_direction = CART_SHAPE.get(char)
            if not cart_direction:
                continue
            cart_id = f'{col}{row}'
            carts[cart_id] = Cart(cart_id, (col, row), cart_direction)
            cart_positions[(col, row)] = cart_id

    part_1 = None
    part_2 = None
    fn_sorter = attrgetter('y', 'x')
    while True:
        if len(carts) == 1:
            part_2 = list(carts.values())[0].position
            break

        for cart_id, cart in sorted(carts.items(), key=lambda x: fn_sorter(x[1])):
            if cart_positions.pop(cart.position, None) is None:
                continue

            cart.tick(grid)

            if cart.position in cart_positions:
                carts.pop(cart_positions.pop(cart.position, None), None)
                carts.pop(cart_id)
                part_1 = cart.position if part_1 is None else part_1
                continue

            cart_positions[cart.position] = cart_id
    return part_1, part_2


part_1, part_2 = solution()
print(f'Part 1: {part_1}')
print(f'Part 2: {part_2}')
