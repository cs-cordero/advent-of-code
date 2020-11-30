#!/bin/python3

import re


def security_through_obscurity(room_codes):
    sector_id_sum = 0
    for line in room_codes:
        m = re.search("([a-z\-]*)(\d+)\[(\w*)\]", line)  # noqa
        room_code, sector_id, check_sum = m.groups()

        ch_list = [[chr(i), 0] for i in range(97, 123)]
        for char in room_code:
            if char == "-":
                continue
            ch_list[ord(char) % 97][1] += 1

        # get the top 5, and sort by alphabetically
        ch_list.sort(key=lambda x: (-x[1], x[0]), reverse=True)
        top_5 = ch_list[-5:]
        top_5.sort(key=lambda x: (-x[1], ord(x[0])))

        if "".join(top[0] for top in top_5) == check_sum:
            sector_id_sum += int(sector_id)

    return sector_id_sum


if __name__ == "__main__":
    sample = [
        "aaaaa-bbb-z-y-x-123[abxyz]",
        "a-b-c-d-e-f-g-h-987[abcde]",
        "not-a-real-room-404[oarel]",
        "totally-real-room-200[decoy]",
    ]
    print("Sample: {}".format(security_through_obscurity(sample)))

    advent_input = []
    with open("aoc_day_04_input.txt", "r") as f:
        for line in f:
            advent_input.append(line)

    print("Challenge: {}".format(security_through_obscurity(advent_input)))
