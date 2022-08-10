import rx
from rrs import rrs
from _rrs import ffi



def gen():
    for i in [1,2,3,4]:
        yield i

try:
    map1 = rrs.map(lambda i: i+1)
    map2 = rrs.map(lambda i: i*2)
    count1 = rrs.count(reduce=True)
    #source = rrs.from_external_source(gen)
    source = rrs.from_external_source(rx.from_([1,2,3,4, 5]))

    pipeline = rrs.create_pipeline()
    print("created")
    print(pipeline)
    rrs.pipeline_add_operator(pipeline, map1)
    print("added map")
    print(pipeline)
    rrs.pipeline_add_operator(pipeline, map2)
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
