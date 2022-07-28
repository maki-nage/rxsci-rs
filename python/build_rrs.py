from cffi import FFI
ffibuilder = FFI()

ffibuilder.cdef("""
    extern "Python" int map_mapper_cbk(int, int);
    extern void* map(void*, int);

    extern const void* create_memory_state_store();

    extern "Python" void from_external_source_cbk(int, int, void*);
    extern void* from_external_source(void*, int64_t);
    extern void external_source_on_next(void* p_source, int32_t i);

    //extern void* for_each();

    extern void* create_pipeline();
    extern void pipeline_add_operator(void* p_pipeline, void* p_op);
    extern void pipeline_run(void* p_pipeline, void* p_source);

    extern "Python" void pipeline_on_next_cbk(int, int);
    extern void pipeline_subscribe(
        void *p_pipeline, void *p_source,
        const void* p_store,
        void *on_next,
        int32_t index
    );

""")

ffibuilder.set_source("_rrs",
"""
     #include "rrs.h"
""",
    include_dirs=[
        '../include',
    ],
    extra_objects=["../target/debug/librrs.a"],
    libraries=['dl'],
    #extra_link_args=['-Wl,-rpath=../target/release/']
)

if __name__ == "__main__":
    ffibuilder.compile(verbose=True)
