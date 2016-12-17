#!/bin/python3
# -*- code: utf-8 -*-

from hashlib import md5


def day17(hashkey):
    queue = [(hashkey, (0, 0))]
    directions = {0:'U', 1:'D', 2:'L', 3:'R'}
    longest_path_length = 0
    shortest_path = None
    while queue:
        next_instructions, position = queue.pop(0)

        # reached bottom right room, store solution results
        if position == (3, 3):
            path = next_instructions[len(hashkey):]
            path_len = len(path)
            longest_path_length = max(longest_path_length, path_len)
            if shortest_path is None or \
               (shortest_path and path_len < len(shortest_path)):
                shortest_path = path
            continue

        # enqueue the next moves
        x, y = position
        candidates = [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
        for i in range(4):
            x, y = candidates[i]
            h = md5(next_instructions).hexdigest()[:4]
            if ((x < 0 or y < 0 or x > 3 or y > 3)
                or not h[i] in ('b','c','d','e','f')):
                continue
            new_hash = next_instructions + directions[i]
            queue.append((new_hash, candidates[i]))

    return shortest_path, longest_path_length


if __name__ == '__main__':
    hashkey = 'yjjvjgan'
    solution = day17(hashkey.encode())
    print('Part 1: {}'.format(solution[0]))
    print('Part 2: {}'.format(solution[1]))