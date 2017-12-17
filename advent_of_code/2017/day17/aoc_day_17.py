def solution():
    step = 314 # puzzle input
    spinlock = [0]
    pos = 1
    after_zero = 1
    for i in range(1, 2018):
        pos = 0 if not spinlock else (pos + step) % len(spinlock)
        spinlock = spinlock[:pos+1] + [i] + spinlock[pos+1:]
        pos = len(spinlock[:pos+1])
    return spinlock[pos+1]

def solution2():
    step = 314
    pos = 1
    after_zero = 1
    for i in range(1, 50000000):
        if (pos + step) % i == 0:
            after_zero = i
        pos = (pos + step) % i + 1
    return after_zero


if __name__ == '__main__':
    print(f'Part One: {solution()}')
    print(f'Part Two: {solution2()}')
