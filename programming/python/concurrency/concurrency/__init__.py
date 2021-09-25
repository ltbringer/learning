import multiprocessing
import time


class SumTask:
    def __init__(self):
        self.counter = 0

    def add_range(self, start, end):
        for i in range(start, end + 1):
            self.counter += i

    def __call__(self):
        return self.counter


def single_thread():
    task = SumTask()
    task.add_range(1, 30000000)
    return task()
