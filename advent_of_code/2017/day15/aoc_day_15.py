"""
From the challenge input:
Generator A starts with 634, with factor 16807
Generator B starts with 301, with factor 48271
"""

def generator(num, factor, multiple=1):
    while True:
        num = (num * factor) % (2**31-1)
        if num % multiple == 0:
            yield bin(num & (2**16-1))

def solution():
    genA = generator(634, 16807)
    genB = generator(301, 48271)
    part_1 = sum(1 if next(genA) == next(genB) else 0 for _ in range(40000000))

    genA = generator(634, 16807, 4)
    genB = generator(301, 48271, 8)
    part_2 = sum(1 if next(genA) == next(genB) else 0 for _ in range(5000000))

    return part_1, part_2

if __name__ == '__main__':
    print(solution())
