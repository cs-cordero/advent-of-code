from functools import partial


class Vector:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __hash__(self):
        return hash((self.x, self.y, self.z))

    def __repr__(self):
        return str((self.x, self.y, self.z))


def solution(particles):
    partial_displacement = partial(displacement, 10000)
    best = None
    for i, particle in enumerate(particles):
        particle_score = 0
        vectors = tuple(map(parse, particle.split(", ")))
        for axis in zip(*vectors):
            particle_score += abs(partial_displacement(*axis))
        if not best or abs(particle_score) < abs(best[1]):
            best = (i, particle_score)
        print(i, particle_score)
    return best


def parse(vector):
    return tuple(map(int, vector[:-1].split("=<")[1].split(",")))


def displacement(t, x, v, a):
    return x + (v * t) + (0.5 * a * (t ** 2))


def solution2(particles):
    p = {}
    for i, particle in enumerate(particles):
        vectors = tuple(map(parse, particle.split(", ")))
        p[i] = [Vector(*vector) for vector in vectors]

    for iteration in range(1000000):
        if iteration % 100000 == 0:
            print("Ticking... ", iteration)
        tick(p)

    import pdb

    pdb.set_trace()


def tick(particle_dict):
    seen = {}
    for_deletion = set()
    for i, particle in particle_dict.items():
        particle[1] += particle[2]
        particle[0] += particle[1]
        if particle[0] in seen:
            for_deletion.add(i)
            for_deletion.add(seen[particle[0]])
        else:
            seen[particle[0]] = i
    for i in for_deletion:
        del particle_dict[i]
    return particle_dict


if __name__ == "__main__":
    with open("aoc_day_20_input.txt", "r") as f:
        s = [line.strip() for line in f.readlines()]
    print(solution(s))
    print(solution2(s))
