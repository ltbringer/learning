from concurrency.thread import sum_


def test_sum_thread():
    value = sum_.single_thread()
    assert value == sum_.multi_thread()
    assert value == sum_.multi_processes()
