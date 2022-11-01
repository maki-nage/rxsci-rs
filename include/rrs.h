#include <stdbool.h>

extern void* map(void*, int);
extern void* count(const void* schema, bool);

extern const void* create_memory_state_store();

extern void* from_external_source(void*, int64_t);
extern void external_source_on_next(void *p_source, const void*);
extern void external_source_on_completed(void *p_source);
//extern void* for_each();

extern void* create_pipeline();
extern void pipeline_add_operator(void *p_pipeline, void *p_op);
extern void pipeline_run(void *p_pipeline, void *p_source);
extern void pipeline_subscribe(
    void *p_pipeline, void *p_source,
    const void* p_store,
    void *on_next,
    int32_t index
);

// flextuple schema
extern void* flextuple_schema_builder(const char* name);
extern void* flextuple_schema_build(const void* p_self);
extern void flextuple_schema_drop(const void* p_self);
extern void flextuple_schema_set_handle(const void* p_self, const void *handle);
extern void flextuple_schema_add_int64(void *p_self, const char* name);

extern const void* flextuple_schema_get_handle(const void* p_self);


// flextuple
extern void* flextuple_builder(const void* p_schema);
extern void* flextuple_build(const void* p_self);
extern void flextuple_drop(const void* p_self);
extern void flextuple_add_int64(void* p_ft, int64_t value);
extern void flextuple_add_float64(void* p_ft, double value);

extern void* flextuple_get_handle(void* p_self);
extern int64_t flextuple_get_int64_at(void* p_ft, size_t index);
extern double flextuple_get_float64_at(void* p_ft, size_t index);
