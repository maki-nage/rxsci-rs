from collections import namedtuple
import rx
from _rrs import ffi, lib

from .flextuple import FlexTuple


class FlexInt(FlexTuple):
    value: int

map_mappers = []
split_mappers = []


@ffi.def_extern()
def map_mapper_cbk(index, i):
    t = ffi.from_handle(lib.flextuple_get_handle(i))
    pi = t()
    pi.init_from_native(i, own=False)

    r =  map_mappers[index](pi)
    return r.__ft

@ffi.def_extern()
def from_external_source_cbk(cmd, index, sink):
    if cmd == 0:  # subscribe
        #print("subscribe from_external_source_cbk {}".format(index))

        def on_next(i):
            #print(f"rx on next: {i}")
            i.__ft = lib.external_source_on_next(sink, i.__ft)
            #print(f"rx on next done: {i}")

        def on_completed():
            #print(f"rx on completed")
            lib.external_source_on_completed(sink)

        source = external_sources[index]
        source.subscribe(
            on_next=on_next,
            on_completed=on_completed,
        )
        #for i in source:
        #    lib.external_source_on_next(sink, i)


def map(mapper):
    map_mappers.append(mapper)
    return lib.map(lib.map_mapper_cbk, len(map_mappers)-1)

def count(reduce):
    return lib.count(FlexInt.__schema__, reduce)


@ffi.def_extern()
def split_mapper_cbk(index, i):
    t = ffi.from_handle(lib.flextuple_get_handle(i))
    pi = t()
    pi.init_from_native(i, own=False)

    r =  split_mappers[index](pi)
    print(f"split mapper result: {r}")
    return r  # r.__ft


def split(mapper, pipeline):
    split_mappers.append(mapper)
    ops =  [
        lib.push_key_split(lib.split_mapper_cbk, len(split_mappers)-1),
    ]
    
    for op in pipeline:
        ops.append(op)

    ops.append(lib.pop_key())
    return ops

def from_external_source(gen):
    external_sources.append(gen)
    return lib.from_external_source(lib.from_external_source_cbk, len(external_sources)-1)


def from_rx(source):
    #external_sources.append(gen)
    external_sources.append(source)
    return lib.from_external_source(lib.from_external_source_cbk, len(external_sources)-1)


def for_each():
    return lib.for_each()

def create_memory_state_store():
    return lib.create_memory_state_store()

def create_pipeline():
    return lib.create_pipeline()

def pipeline_add_operator(pipeline, op):
    if type(op) is list:
        for o in op:
            lib.pipeline_add_operator(pipeline, o)
    else:
        lib.pipeline_add_operator(pipeline, op)

def pipeline_run(pipeline, source):
    lib.pipeline_run(pipeline, source)


Observer = namedtuple('Observer', ['on_next', 'on_error', 'on_completed'])
external_sources = []
external_sinks = []

@ffi.def_extern()
def pipeline_on_next_cbk(index, i):
    #print('pipeline_on_next_cbk')
    observer = external_sinks[index]
    #print(observer)
    if observer.on_next is not None:
        t = ffi.from_handle(lib.flextuple_get_handle(i))
        pi = t()
        pi.init_from_native(i)

        observer.on_next(pi)


def compose(pipeline, ops):
    for op in ops:
        if type(op) is list:
            compose(pipeline, op)
        else:
            lib.pipeline_add_operator(pipeline, op)
    
    return pipeline

def pipeline_subscribe(pipeline, source, state_store, on_next=None, on_error=None, on_completed=None):
    external_sinks.append(Observer(
        on_next=on_next,
        on_error=on_error,
        on_completed=on_completed,
    ))
    lib.pipeline_subscribe(
        pipeline, source, state_store,
        lib.pipeline_on_next_cbk, 
        len(external_sinks)-1
    )
