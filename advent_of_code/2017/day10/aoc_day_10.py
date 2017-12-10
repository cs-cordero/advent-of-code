
def solution(stream):
    arr = list(range(256))
    current_position = 0
    skip_size = 0

    lengths = parse_input_as_int(stream)
    for length in lengths:
        arr = reverse_length(arr, length, current_position)
        current_position = move(current_position, length, skip_size, len(arr))
        skip_size += 1
    return arr

def solution_part2(stream):
    arr = list(range(256))
    current_position = 0
    skip_size = 0

    lengths = list(parse_input_as_bytes(stream))
    lengths.extend([17, 31, 73, 47, 23])
    for _ in range(64):
        for length in lengths:
            arr = reverse_length(arr, length, current_position)
            current_position = move(current_position, length, skip_size, len(arr))
            skip_size += 1

    arr = get_dense_hash(arr)
    knot_hash = get_hex_hash(arr)
    return knot_hash

def parse_input_as_int(stream):
    return map(int, stream.split(','))

def parse_input_as_bytes(stream):
    return map(ord, stream)

def reverse_length(arr, length, current_position):
    mod = len(arr)
    left = current_position
    right = current_position + length - 1
    while left < right:
        arr[left%mod], arr[right%mod] = arr[right%mod], arr[left%mod]
        left += 1
        right -= 1
    return arr

def move(current_position, length, skip_size, mod_length):
    current_position += length + skip_size
    return current_position

def get_dense_hash(arr):

    dense_hash = []
    def perform_xor(arr):
        result = 0
        for num in arr:
            result ^= num
        return result

    for i in range(0, len(arr), 16):
        dense_hash.append(perform_xor(arr[i:i+16]))

    return dense_hash

def get_hex_hash(arr):
    hex_hash = ''
    for dense_num in arr:
        hex_hash += '{:02x}'.format(dense_num)
    return hex_hash

if __name__ == '__main__':
    with open('aoc_day_10_input.txt', 'r') as f:
        s = f.readlines()[0].strip()
    part_one = solution(s)
    part_two = solution_part2(s)
    print(part_one[0] * part_one[1])
    print(part_two)
