from functools import reduce
from collections import deque

puzzle_input = 'amgozmfv'

def solution(s):
    grid = []
    sum_ones = 0
    for i in range(128):
        khash = get_knot_hash(s + f'-{i}')
        khash_bin = ''.join(bin(int(hx, 16))[2:].zfill(4) for hx in khash)
        grid.append([int(x) for x in khash_bin])
        sum_ones += sum(grid[-1])

    queue = deque()
    regions = 0
    for x in range(128):
        for y in range(128):
            if grid[x][y] > 0:
                queue.append((x, y))
                regions += 1

            while queue:
                current_x, current_y = queue.popleft()
                if grid[current_x][current_y] == 0:
                    continue
                grid[current_x][current_y] = 0

                if current_x - 1 >= 0:
                    if grid[current_x-1][current_y] == 1:
                        queue.append((current_x-1, current_y))
                if current_x + 1 < 128:
                    if grid[current_x+1][current_y] == 1:
                        queue.append((current_x+1, current_y))
                if current_y - 1 >= 0:
                    if grid[current_x][current_y-1] == 1:
                        queue.append((current_x, current_y-1))
                if current_y + 1 < 128:
                    if grid[current_x][current_y+1] == 1:
                        queue.append((current_x, current_y+1))

    return sum_ones, regions


def get_knot_hash(s):
    def reverse_length(arr, length, current_position):
        mod = len(arr)
        left = current_position
        right = current_position + length - 1
        while left < right:
            arr[left%mod], arr[right%mod] = arr[right%mod], arr[left%mod]
            left += 1
            right -= 1
        return arr

    arr = list(range(256))
    lengths = [ord(char) for char in s] + [17, 31, 73, 47, 23]
    position = 0
    skip = 0
    for _ in range(64):
        for length in lengths:
            arr = reverse_length(arr, length, position)
            position = (position + length + skip) % 256
            skip += 1

    dense_hash = [reduce(lambda a, b: a^b, arr[i:i+16])
                  for i in range(0, 256, 16)]
    return ''.join('{:02x}'.format(num) for num in dense_hash)

if __name__ == '__main__':
    answer = solution(puzzle_input)
    print(f'Part One: {answer[0]}')
    print(f'Part Two: {answer[1]}')
