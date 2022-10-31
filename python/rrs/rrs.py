from collections import namedtuple
import rx
from _rrs import ffi, lib

from .flextuple import FlexTuple


class FlexInt(FlexTuple):
    value: int

map_mappers = []


@ffi.def_extern()
def map_mapper_cbk(index, i):
    wrap = lib.flextuple_build_from_native(i)
    t = ffi.from_handle(lib.flextuple_get_handle(wrap))
    i = t()
    i.init_from_native(wrap)

    r =  map_mappers[index](i)
    return r.__ft

@ffi.def_extern()
def from_external_source_cbk(cmd, index, sink):
    if cmd == 0:  # subscribe
        #print("subscribe from_external_source_cbk {}".format(index))

        def on_next(i):
            #print(f"rx on next: {i}")
            lib.external_source_on_next(sink, i.__ft)
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
        wrap = lib.flextuple_build_from_native(i)
        t = ffi.from_handle(lib.flextuple_get_handle(wrap))
        i = t()
        i.init_from_native(wrap)

        observer.on_next(i)


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
