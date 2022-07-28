import rx
from rrs import rrs
from _rrs import ffi



def gen():
    for i in [1,2,3,4]:
        yield i

try:
    map1 = rrs.map(lambda i: i+1)
    map2 = rrs.map(lambda i: i*2)
    #source = rrs.from_external_source(gen)
    source = rrs.from_external_source(rx.from_([1,2,3,4]))

    pipeline = rrs.create_pipeline()
    print("created")
    print(pipeline)
    rrs.pipeline_add_operator(pipeline, map1)
    print("added map")
    print(pipeline)
    rrs.pipeline_add_operator(pipeline, map2)
    print("added map2")

    #print(pipeline)
    #rrs.pipeline_run(pipeline, source)

    def on_next(i):
        print('nnnn {}'.format(i))

    state_store = rrs.create_memory_state_store()
    rrs.pipeline_subscribe(pipeline, source, state_store, on_next)
except Exception as e:
    print(e)
