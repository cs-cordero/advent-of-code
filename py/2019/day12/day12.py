from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import List


@dataclass
class Vector3D:
    x: int = 0
    y: int = 0
    z: int = 0

    def __iadd__(self, other: Vector3D) -> Vector3D:
        self.x += other.x
        self.y += other.y
        self.z += other.z
        return self


def solution1(data: List[int]) -> int:
    position = deepcopy(data)
    velocity = [Vector3D() for _ in position]

    steps = 1000
    for step in range(steps):
        for i in range(len(position) - 1):
            for j in range(i + 1, len(position)):
                moon_i = position[i]
                moon_j = position[j]
                if moon_i.x < moon_j.x:
                    velocity[i].x += 1
                    velocity[j].x -= 1
                elif moon_i.x > moon_j.x:
                    velocity[i].x -= 1
                    velocity[j].x += 1
                if moon_i.y < moon_j.y:
                    velocity[i].y += 1
                    velocity[j].y -= 1
                elif moon_i.y > moon_j.y:
                    velocity[i].y -= 1
                    velocity[j].y += 1
                if moon_i.z < moon_j.z:
                    velocity[i].z += 1
                    velocity[j].z -= 1
                elif moon_i.z > moon_j.z:
                    velocity[i].z -= 1
                    velocity[j].z += 1
        for k in range(len(position)):
            position[k] += velocity[k]

    total_energy = 0
    for pos, vel in zip(position, velocity):
        total_energy += (abs(pos.x) + abs(pos.y) + abs(pos.z)) * (
            abs(vel.x) + abs(vel.y) + abs(vel.z)
        )
    return total_energy


def solution2(data: List[int]) -> int:
    def get_steps_until_cycle(positions: List[int]) -> int:
        velocities = [0] * len(positions)
        originals = positions.copy()
        steps = 0
        while True:
            for i in range(len(positions) - 1):
                position_i = positions[i]
                for j in range(i + 1, len(positions)):
                    position_j = positions[j]
                    if position_i < position_j:
                        velocities[i] += 1
                        velocities[j] -= 1
                    elif position_i > position_j:
                        velocities[i] -= 1
                        velocities[j] += 1

            for k, vel in enumerate(velocities):
                positions[k] += vel

            steps += 1

            if positions == originals and all(velocity == 0 for velocity in velocities):
                break
        return steps

    def find_gcf(a: int, b: int) -> int:
        return find_gcf(b, a % b) if a % b == 0 else b

    def find_lcm(a: int, b: int) -> int:
        return (a * b) // find_gcf(a, b)

    steps_x = get_steps_until_cycle([pos.x for pos in data])
    steps_y = get_steps_until_cycle([pos.y for pos in data])
    steps_z = get_steps_until_cycle([pos.z for pos in data])
    return find_lcm(find_lcm(steps_x, steps_y), steps_z)


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = []
        for line in f.readlines():
            x, y, z = line[1:-2].strip().split(", ")
            x = int(x[2:])
            y = int(y[2:])
            z = int(z[2:])
            data.append(Vector3D(x, y, z))

    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
