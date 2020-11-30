from collections import defaultdict
from datetime import datetime, timedelta


def read_file(filepath):
    with open(filepath) as f:
        for line in f.readlines():
            yield line.strip()


def solution():
    records = []
    for line in read_file("input.txt"):
        datetime_str, description = line[1:].split("] ")
        dt = datetime.strptime(datetime_str, "%Y-%m-%d %H:%M")
        records.append((dt, description))

    records = sorted(records, key=lambda x: x[0])
    current_guard = None
    sleep_ts = None
    times = {}
    for dt, description in records:
        if "begins shift" in description:
            guard, guard_id, begins, shift = description.split(" ")
            current_guard = guard_id
        elif "falls asleep" == description:
            if sleep_ts is not None:
                continue
            sleep_ts = dt
        elif "wakes up" == description:
            if sleep_ts is None:
                continue
            time_spent_sleeping = (dt - sleep_ts).seconds / 60
            if current_guard not in times:
                times[current_guard] = {
                    "total_time_spent_sleeping": 0,
                    "minute_dict": defaultdict(int),
                }
            times[current_guard]["total_time_spent_sleeping"] += time_spent_sleeping
            while sleep_ts < dt:
                times[current_guard]["minute_dict"][sleep_ts.minute] += 1
                sleep_ts += timedelta(minutes=1)
            sleep_ts = None

    highest_freq = (None, 0, 0)
    for guard, data in times.items():
        minutes = sorted(data["minute_dict"].items(), key=lambda x: x[1])
        if minutes[-1][1] > highest_freq[-1]:
            highest_freq = (guard, minutes[-1][0], minutes[-1][1])
    return highest_freq


print(solution())
