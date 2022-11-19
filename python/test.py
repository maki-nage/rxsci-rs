import rx
from rrs import rrs, FlexTuple
from rrs.rrs import FlexInt
from _rrs import ffi

class Item(FlexTuple):
    index: int
    value: float

def gen():
    for i in [1, 1, 1, 2, 2, 3]:
        yield Item(i, float(i/2))


def test_base():
    try:
        map1 = rrs.map(lambda i: i)
        #map2 = rrs.map(lambda i: i)
        count1 = rrs.count(reduce=False)
        print("created count")
        #source = rrs.from_external_source(gen)
        source = rrs.from_external_source(rx.from_(gen()))

        pipeline = rrs.create_pipeline()
        print("created")
        print(pipeline)
        rrs.pipeline_add_operator(pipeline, map1)
        #print("added map")
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


def test_split():
    try:
        ops = [
            rrs.split(lambda i: FlexInt(i.index), pipeline=[
                rrs.split(lambda i: FlexInt(i.index), pipeline=[
                    rrs.count(reduce=True),
                ])
            ])
        ]
        source = rrs.from_external_source(rx.from_(gen()))

        pipeline = rrs.create_pipeline()

        def on_next(i):
            print('sss {}'.format(i))

        def on_completed():
            print("on_completed")

        state_store = rrs.create_memory_state_store()        
        rrs.pipeline_subscribe(
            rrs.compose(pipeline, ops),
            source, state_store,
            on_next=on_next,
            on_completed=on_completed,
        )
    except Exception as e:
        print(e)


test_base()
test_split()
