import rx
from rrs import rrs, FlexTuple
from _rrs import ffi

class Item(FlexTuple):
    index: int
    value: float

def gen():
    for i in [1,2,3,4]:
        yield Item(i, float(i/2))

try:
    #map1 = rrs.map(lambda i: i)
    #map2 = rrs.map(lambda i: i)
    count1 = rrs.count(reduce=True)
    print("created count")
    #source = rrs.from_external_source(gen)
    source = rrs.from_external_source(rx.from_(gen()))

    pipeline = rrs.create_pipeline()
    print("created")
    print(pipeline)
    #rrs.pipeline_add_operator(pipeline, map1)
    #print("added map")
    print(pipeline)
    #rrs.pipeline_add_operator(pipeline, map2)
    rrs.pipeline_add_operator(pipeline, count1)
    print("added count")

    #print(pipeline)
    #rrs.pipeline_run(pipeline, source)

    def on_next(i):
        print('nnnn {}'.format(i))

    def on_completed():
        print("on_completed")

    state_store = rrs.create_memory_state_store()
    rrs.pipeline_subscribe(
        pipeline, source, state_store,
        on_next=on_next,
        on_completed=on_completed,
    )
except Exception as e:
    print(e)
