import time
import os

def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def solution():
    points = []
    velocities = []
    for line in read_file('input.txt'):
        left, right = line.split(' velocity=<')
        position = list(map(int, left[10:-1].split(', ')))
        velocity = list(map(int, right[:-1].split(', ')))
        points.append(position)
        velocities.append(velocity)

    elapsed_seconds = 0
    while True:
        all_xs, all_ys = zip(*points)
        minx, maxx, miny, maxy = min(all_xs), max(all_xs), min(all_ys), max(all_ys)
        if abs(maxx - minx) < 100 and abs(maxy - miny) < 100:
            grid = [
                ['.' for _x in range((minx*-1)+abs(maxx)+1)]
                for _y in range((miny*-1)+abs(maxy)+1)
            ]
            for x, y in points:
                grid[y-miny][x-minx] = '#'

            os.system('clear')
            for row in grid:
                print(''.join(row))

            # 10 was manually determined by eyeballing the output
            if len(grid) == 10:
                return f'Completed in {elapsed_seconds} seconds.'
            time.sleep(1)

        for i, velocity in enumerate(velocities):
            points[i][0] += velocity[0]
            points[i][1] += velocity[1]
        elapsed_seconds += 1

print(solution())
