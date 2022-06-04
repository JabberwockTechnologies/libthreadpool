
// Thread ID in current work group.
thread_local!(#[allow(non_upper_case_globals)] static libthreadpool_local_tid: i32 = i32::MAX);

#[no_mangle]
pub unsafe extern "C" fn libthreadpool_get_local_tid() -> i32 {
    let mut tid = 0i32;
    libthreadpool_local_tid.with(|t| {
        tid = *t;
    });
    return tid;
}

