from concurrency.thread.race import eval_random_gen


def test_shows_multiple_of_5_not_returned_due_to_racing_threads():
    n = 5
    start = 0
    end = 50
    assert eval_random_gen(n, start=start, end=end) % n != 0
