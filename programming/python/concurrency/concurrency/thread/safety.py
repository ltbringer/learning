from threading import Thread, Lock
from typing import Union
from concurrency.utils import t


class UnsafeCounter:
    def __init__(self, limit=0):
        self.acc = 0
        self.limit = limit

    def increment(self):
        for _ in range(self.limit):
            self.acc += 1


class SafeCounter:
    def __init__(self, limit=0):
        self.acc = 0
        self.limit = limit
        self.lock = Lock()

    def increment(self):
        with self.lock:
            for _ in range(self.limit):
                    self.acc += 1

@t
def eval_counter(obj: Union[SafeCounter, UnsafeCounter], n_threads=0):
    threads = [Thread(target=obj.increment) for _ in range(n_threads)]
    for thread in threads:
        thread.start()

    for thread in threads:
        thread.join()

    return obj.acc
