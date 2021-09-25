from concurrency.thread import sum_thread


def test_sum_thread():
    value = sum_thread.single_thread()
    assert value == sum_thread.multi_thread()
    assert value == sum_thread.multi_processes()
