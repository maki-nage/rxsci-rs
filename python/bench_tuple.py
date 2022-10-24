from typing import NamedTuple
from collections import namedtuple
import time


class Item(NamedTuple):
    foo: int
    bar: int
    buzz: int    

#Item = namedtuple('Item', ['foo', 'bar', 'buzz'])
run_count = 10000000

# named access
time_start = time.time()
for _ in range(run_count):
    i = Item(
        foo=42,
        bar=34,
        buzz=3
    )
    i = i._replace(buzz=4)
    a = i.foo * i.bar + i.buzz
    b = i.foo / i.bar - i.buzz
time_end = time.time()
time_field = (time_end - time_start) / run_count

time_start = time.time()
for _ in range(run_count):
    i = Item(
        foo=42,
        bar=34,
        buzz=3
    )
    i = i._replace(buzz=4)
    a = i[0] * i[1] + i[2]
    b = i[0] / i[1] - i[2]
time_end = time.time()
time_index = (time_end - time_start) / run_count

time_start = time.time()
for _ in range(run_count):
    i = Item(42, 34, 3)
    i = i._make((i[0], i[1], 4))
    a = i[0] * i[1] + i[2]
    b = i[0] / i[1] - i[2]
time_end = time.time()
time_fullindex = (time_end - time_start) / run_count


print(f"field time: {time_field}")
print(f"index time: {time_index}")
print(f"fullindex time: {time_fullindex}")

print(isinstance(Item(1,2,3), tuple))
