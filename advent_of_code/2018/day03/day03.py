def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def solution():
    grid = [[0 for x in range(1000)] for y in range(1000)]
    grid_summary = {}
    overlapped_square_count = 0
    for line in read_file('input.txt'):
        claim_id, rect_details = line.split(' @ ')
        coordinates, size = rect_details.split(': ')
        left, top = [int(x) for x in coordinates.split(',')]
        width, height = [int(x) for x in size.split('x')]
        claim_id_int = int(claim_id[1:])
        grid_summary[claim_id_int] = (left, top, width, height)

        for x in range(left, left + width):
            for y in range(top, top + height):
                if grid[x][y] == 0:
                    grid[x][y] = claim_id_int
                elif grid[x][y] == 'X':
                    continue
                else:
                    overlapped_square_count += 1
                    grid[x][y] = 'X'

    for rect, details in grid_summary.items():
        left, top, width, height = details
        if all(grid[x][y] != 'X'
               for x in range(left, left + width)
               for y in range(top, top + height)):
            return overlapped_square_count, rect


print(solution())
