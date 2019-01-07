from collections import deque
from copy import deepcopy
from typing import NamedTuple, Tuple, Iterable, Union, Dict, Optional, List, Set
import enum
import os
import time


class EntityType(enum.Enum):
    value: str
    GOBLIN = 'G'
    ELF = 'E'
    WALL = '#'


class Entity:
    def __init__(self, _type: EntityType) -> None:
        self.type = _type
        self.power = 0 if _type is EntityType.WALL else 3
        self.hp = 0 if _type is EntityType.WALL else 200

    def __repr__(self) -> str:
        return self.type.value

    @property
    def is_alive(self) -> bool:
        return self.type is not EntityType.WALL and self.hp > 0

    @property
    def is_blocking(self) -> bool:
        return self.type is EntityType.WALL or self.is_alive


class Point(NamedTuple):
    x: int
    y: int

    def __add__(self, other: Tuple[int, int]) -> 'Point':  # type: ignore
        x1, y1 = self
        x2, y2 = other
        return Point(x1+x2, y1+y2)

    @property
    def adjacent(self) -> Iterable['Point']:
        for delta in [(0, -1), (-1, 0), (1, 0), (0, 1)]:
            yield self + delta

    @staticmethod
    def coerce(point: 'GridKey') -> 'Point':
        if isinstance(point, Point) and type(point) == Point:
            return point
        return Point(*point)


GridKey = Union[Point, Tuple[int, int]]


class Grid:
    def __init__(self, raw_lines: str, elf_power: int) -> None:
        self._game_over = False
        self._grid: Dict[GridKey, Entity] = {}
        for row_num, row in enumerate(raw_lines.strip().split('\n')):
            for col_num, col in enumerate(row.strip()):
                if col == '.':
                    continue
                entity_type = EntityType(col)
                entity = Entity(entity_type)
                if entity_type is EntityType.ELF:
                    entity.power = elf_power
                self._grid[(col_num, row_num)] = entity

    @property
    def live_entities(self) -> Dict[Point, Entity]:
        return {
            Point.coerce(point): entity
            for point, entity in self._grid.items()
            if entity.is_alive
        }

    @property
    def turn_order(self) -> List[Tuple[Point, Entity]]:
        items: Iterable[Tuple[Point, Entity]] = self.live_entities.items()
        return sorted(items, key=lambda item: (item[0][1], item[0][0]))

    @property
    def game_over(self) -> bool:
        if not self._game_over:
            types_remaining = {entity.type for entity in self.live_entities.values()}
            self._game_over = len(types_remaining) == 1
        return self._game_over

    @property
    def outcome(self) -> Tuple[EntityType, int]:
        assert self.game_over, 'Game is still in progress. Keep calling play().'
        live_entities = list(self.live_entities.values())

        victors = live_entities[0].type
        total_health = sum(entity.hp for entity in live_entities)
        return (victors, total_health)

    def is_unblocked(self, key: GridKey) -> bool:
        entity = self._grid.get(key)
        return entity is None or not entity.is_blocking

    def get_attack_target_from(self, position: GridKey) -> Optional[Point]:
        position = Point.coerce(position)
        entity = self[position]
        assert entity is not None
        live_entities = self.live_entities

        target_position = None
        target = None
        for adjacent_position in position.adjacent:
            adjacent_entity = live_entities.get(adjacent_position)
            if adjacent_entity is None:
                continue
            if not adjacent_entity.is_alive:
                continue
            if adjacent_entity.type == entity.type:
                continue
            if target is None or target.hp > adjacent_entity.hp:
                target = adjacent_entity
                target_position = adjacent_position
        return target_position

    def get_targets(self, starting_point: GridKey) -> Set[Point]:
        starting_point = Point.coerce(starting_point)
        starting_entity = self[starting_point]
        assert starting_entity is not None
        assert starting_entity.is_alive

        starting_point_type = starting_entity.type

        targets = set()
        for position, entity in self.live_entities.items():
            position = Point(*position)
            if position == starting_point:
                continue
            if entity.type == starting_point_type:
                continue
            for adjacent_position in position.adjacent:
                if self.is_unblocked(adjacent_position):
                    targets.add(adjacent_position)
        return targets

    def bfs(self, mover: GridKey) -> Optional[List[Point]]:
        targets = self.get_targets(mover)
        if not targets:
            return None

        mover = Point.coerce(mover)
        queue = deque([[mover]])
        seen = {mover}
        best = None
        while queue:
            path = queue.popleft()
            current_position = path[-1]
            for position in current_position.adjacent:
                if position in targets:
                    seen.add(position)
                    if best is None:
                        best = path + [position]
                        continue

                    current = path + [position]
                    if len(current) > len(best):
                        continue
                    elif len(current) < len(best):
                        best = current
                        continue

                    # Must get the target that is first in 'reading order'
                    bestx, besty = best[-1]
                    currentx, currenty = current[-1]
                    if (currenty < besty
                            or (currenty == besty and currentx < bestx)):
                        best = current
                        continue

                if position in seen:
                    continue

                if self.is_unblocked(position):
                    queue.append(path + [position])
                    seen.add(position)
        return best

    def play(self, verbose: bool = False) -> bool:
        for position, entity in self.turn_order:
            if self.game_over:
                if verbose:
                    os.system('clear')
                    print(self)
                return self.game_over

            if not entity.is_alive:
                continue
            attack_target = self.get_attack_target_from(position)
            if not attack_target:
                path = self.bfs(position)
                if not path:
                    continue
                current_position, next_position, *rest = path
                assert current_position == position, 'Just a sanity check'
                del self._grid[position]
                self[next_position] = entity
                attack_target = self.get_attack_target_from(next_position)

            if attack_target:
                defending_entity = self[attack_target]
                assert defending_entity is not None
                defending_entity.hp -= entity.power
        if verbose:
            os.system('clear')
            print(self)
        return False

    def __getitem__(self, key: GridKey) -> Entity:
        return self._grid[key]

    def __setitem__(self, key: GridKey, value: Entity) -> None:
        self._grid[key] = value

    def copy(self) -> 'Grid':
        return deepcopy(self)

    def show_turn_order(self) -> None:
        _grid = self.copy()
        for i, turn in enumerate(_grid.turn_order):
            _grid[turn] = i+1  # type: ignore
        print(repr(_grid))

    def show_targets(self, targeter: GridKey) -> None:
        _grid = self.copy()
        for target in self.get_targets(targeter):
            _grid[target] = '!'  # type: ignore
        print(repr(_grid))

    def show_path(self, mover: GridKey) -> None:
        _grid = self.copy()
        path = self.bfs(mover)
        assert path is not None
        for target in path[1:]:
            _grid[target] = 'x'  # type: ignore
        print(repr(_grid))

    def as_string(self) -> str:
        representation = repr(self)
        return ''.join(representation.split('\n'))

    def __repr__(self) -> str:
        xs, ys = zip(*self._grid.keys())
        maxx, maxy = max(xs), max(ys)
        grid = []
        for y in range(maxy+1):
            row = ''
            for x in range(maxx+1):
                entity = self._grid.get((x, y))
                character = (
                    '.' if entity is None or (not isinstance(entity, str)
                                              and not entity.is_alive
                                              and entity.type is not EntityType.WALL)
                    else str(entity)
                )
                row += character
            grid.append(row)
        return '\n'.join(grid)


def get_elf_count(grid: Grid) -> int:
    return sum(entity.type is EntityType.ELF
               for entity in grid.live_entities.values())


def solution(grid_data: str, elf_power: int = 3,
             allow_no_elf_death: bool = False,
             verbose: bool = False) -> Tuple[EntityType, int]:
    grid = Grid(grid_data, elf_power)
    starting_elf_count = get_elf_count(grid)
    game_round = 0
    while not grid.game_over:
        if verbose:
            time.sleep(0.1)
        game_round += not grid.play(verbose)
        if allow_no_elf_death and get_elf_count(grid) < starting_elf_count:
            return EntityType.GOBLIN, game_round

    victors, remaining_health = grid.outcome
    return victors, game_round*remaining_health


def solution2(grid_data: str) -> int:
    for elf_power in range(4, 300):
        victor, remaining_health = solution(grid_data, elf_power, allow_no_elf_death=True)
        if victor == EntityType.ELF:
            return remaining_health
    assert False, 'Did not find a power where the elves won.'


with open('input.txt') as f:
    actual_initial = ''.join(f.readlines())
print(f'Part 1: {solution(actual_initial, verbose=True)[1]}')
print(f'Part 2: {solution2(actual_initial)}')
