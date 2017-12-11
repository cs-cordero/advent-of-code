def solution(lines):
    checksum = 0 # part 1
    checksum2 = 0 # part 2
    for line in lines:
        lline = sorted(list(line))
        checksum += max(lline) - min(lline)
        hi, lo = find_divisible(lline)
        checksum2 += lline[hi] / lline[lo]
    return checksum, checksum2

def find_divisible(line):
    for i in range(len(line)-1, 0, -1):
        j = 0
        while line[j] <= line[i] // 2:
            if line[i] % line[j] == 0:
                return i, j
            j += 1
    raise Exception('could not find divisible')


if __name__ == '__main__':
    with open('aoc_day02_input.txt') as f:
        s = [map(int, line.strip().split('\t'))
             for line in f.readlines()]
    print(solution(s))
