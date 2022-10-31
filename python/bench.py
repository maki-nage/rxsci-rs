from typing import NamedTuple
import time
import rx
import rx.operators as ops
import rxsci as rs

from rrs import rrs, FlexTuple

class ItemNT(NamedTuple):
    index: int
    value: float

class Item(FlexTuple):
    index: int
    value: float

def gen():
    for i in range(30000):
        yield Item(i, float(i/2))

def gen_nt():
    for i in range(30000):
        yield ItemNT(i, float(i/2))

run_count = 5

# run python backend
time_start = time.time()
for _ in range(run_count):
    rx.from_(gen_nt()).pipe(
        #ops.map(lambda i: i*2),
        rs.state.with_memory_store(pipeline=[
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=False),
            rs.ops.count(reduce=True),
        ])
    ).run()
time_end = time.time()
time_py = (time_end - time_start) / run_count

# run rust backend
time_start = time.time()


def on_next(i):
    pass

for _ in range(run_count):
    source = rrs.from_external_source(rx.from_(gen()))

    pipeline = rrs.create_pipeline()
    #rrs.pipeline_add_operator(pipeline, rrs.map(lambda i: i*2))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=False))
    rrs.pipeline_add_operator(pipeline, rrs.count(reduce=True))

    state_store = rrs.create_memory_state_store()
    rrs.pipeline_subscribe(
        pipeline, source, state_store,
        on_next=on_next,
    )
time_end = time.time()
time_rust = (time_end - time_start) / run_count


print(f"python time: {time_py}")
print(f"rust time: {time_rust}")
