import sys
from concurrency.thread.safety import SafeCounter, UnsafeCounter, eval_counter

MAX_THREADS = 5
INC_LIMIT = 100000

sys.setswitchinterval(0.0005)

def test_thread_unsafety():
    unsafe_counter = UnsafeCounter(limit=INC_LIMIT)
    result = eval_counter(unsafe_counter, n_threads=MAX_THREADS)
    assert result < MAX_THREADS * INC_LIMIT


def test_thread_safety():
    safe_counter = SafeCounter(limit=INC_LIMIT)
    result = eval_counter(safe_counter, n_threads=MAX_THREADS)
    assert result == MAX_THREADS * INC_LIMIT
