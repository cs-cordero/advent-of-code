from typing import Dict


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.rstrip()


TARGET_SUE_DETAILS = {
    "children: 3",
    "cats: 7",
    "samoyeds: 2",
    "pomeranians: 3",
    "akitas: 0",
    "vizslas: 0",
    "goldfish: 5",
    "trees: 3",
    "cars: 2",
    "perfumes: 1",
}


def solution():
    for line in read_file("aoc_day_16_input.txt"):
        sue_no, details = line.split(": ", 1)
        if set(details.split(", ")).issubset(TARGET_SUE_DETAILS):
            return sue_no


def solution2():
    actual_target_sue_details = {}
    for detail in TARGET_SUE_DETAILS:
        key, value = detail.split(": ")
        actual_target_sue_details[key] = value

    sues = {}
    for line in read_file("aoc_day_16_input.txt"):
        sue_no, details = line.split(": ", 1)
        local_sue_details = {}
        for local_detail in details.split(", "):
            key, value = local_detail.split(": ")
            local_sue_details[key] = value
        sues[sue_no] = local_sue_details

    def sue_is_valid(sue: Dict[str, int]) -> bool:
        for item, value in sue.items():
            if item in ("cats", "trees"):
                if value <= actual_target_sue_details[item]:
                    return False
            elif item in ("pomeranians", "goldfish"):
                if value >= actual_target_sue_details[item]:
                    return False
            elif value != actual_target_sue_details[item]:
                return False
        return True

    return {sue: detail for sue, detail in sues.items() if sue_is_valid(detail)}


print(solution())
print(solution2())
