def solution(depths):
    severity = 0
    delay = 0

    while True:
        for picosecond in sorted(depths.keys()):
            current_depth = depths[picosecond]
            scan_position = determine_position(current_depth, picosecond + delay)
            if scan_position == 0:
                if not delay == 0:  # calculate severity for part 1
                    break
                severity += picosecond * current_depth
        else:
            if not delay == 0:
                break
        delay += 1

    return severity, delay


def determine_position(depth, picos):
    cycle = (depth - 1) * 2
    offset = picos % cycle
    return offset if offset <= depth - 1 else offset - (((offset % depth) + 1) * 2)


if __name__ == "__main__":
    with open("aoc_day_13_input.txt", "r") as f:
        s = {int(k): int(v) for k, v in (x.strip().split(": ") for x in f.readlines())}
    answer = solution(s)
    print(f"Part One: {answer[0]}")
    print(f"Part Two: {answer[1]}")
