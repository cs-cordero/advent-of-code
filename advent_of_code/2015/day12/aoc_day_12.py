from typing import Generator, Union, Iterable, Dict, List, Optional, Any, cast
import json


JsonItem = Union[int, str, Dict[str, Any], List[Any]]


def solution(test_data: Optional[str] = None, part2: bool = False):
    with open('aoc_day_12_input.txt') as f:
        json_unparsed = f.read()

    json_unparsed = test_data if test_data else json_unparsed
    data = json.loads(json_unparsed)

    def get_numbers(collection: JsonItem) -> Generator[int, None, None]:
        if isinstance(collection, int) and type(collection) == int:
            yield collection
        elif isinstance(collection, str) and type(collection) == str:
            try:
                yield int(collection, 10)
            except ValueError:
                return
        elif isinstance(collection, dict) and type(collection) == dict:
            if part2 and any(value == 'red' for value in collection.values()):
                return
            for key, value in collection.items():
                yield from get_numbers(key)
                yield from get_numbers(value)
        elif isinstance(collection, list) and type(collection) == list:
            for item in collection:
                yield from get_numbers(item)
        else:
            assert False, f'Invariant: {collection} {type(collection)}'

    part1 = sum(number for number in get_numbers(data))
    return part1


assert solution('[1,2,3]') == 6
assert solution('{"a":2,"b":4}') == 6
assert solution('[[[3]]]') == 3
assert solution('{"a":{"b":4},"c":-1}') == 3
assert solution('{"a":[-1,1]}') == 0
assert solution('[-1,{"a":1}]') == 0
assert solution('[]') == 0
assert solution('{}') == 0
print(solution())

assert solution('[1,2,3]', part2=True) == 6
assert solution('[1,{"c":"red","b":2},3]', part2=True) == 4
assert solution('{"d":"red","e":[1,2,3,4],"f":5}', part2=True) == 0
assert solution('[1,"red",5]', part2=True) == 6
print(solution(part2=True))
