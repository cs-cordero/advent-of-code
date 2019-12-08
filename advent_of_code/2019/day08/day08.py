from typing import List


def solution1(data: List[int]) -> int:
    length = 25 * 6
    layers = [data[i : i + length] for i in range(0, len(data), length)]
    layers.sort(key=lambda layer: layer.count(0))
    return layers[0].count(1) * layers[0].count(2)


def solution2(data: List[int]) -> object:
    length = 25 * 6

    layers = [
        [data[j : j + 25] for j in range(i, i + length, 25)]
        for i in range(0, len(data), length)
    ]

    rasterized_layers = []
    for row in zip(*layers):
        rasterized_row = []
        for values in zip(*row):
            for value in values:
                if value == 2:
                    continue
                rasterized_row.append("x" if value == 1 else " ")
                break
        rasterized_layers.append(rasterized_row)

    class ForPrettyPrinting:
        def __str__(self) -> str:
            concatenated = "\n".join(
                "".join(map(str, row)) for row in rasterized_layers
            )
            return f"\n{concatenated}"

    return ForPrettyPrinting()


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = list(map(int, f.readline().strip()))
    print(f"Solution 1: {solution1(data)}")
    print(f"Solution 2: {solution2(data)}")
