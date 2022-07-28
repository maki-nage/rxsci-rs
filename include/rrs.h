extern void* map(void*, int);

extern const void* create_memory_state_store();

extern void* from_external_source(void*, int64_t);
extern void external_source_on_next(void *p_source, int32_t i);
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
