from multiprocessing import Process
from multiprocessing.managers import BaseManager
from threading import Thread

from concurrency.utils import t

START = 1
END = 30000000
MID = END // 2


class SumTask:
    def __init__(self):
        self.counter = 0

    def add_from(self, start: int, end: int):
        for i in range(start, end + 1):
            self.counter += i

    def get_sum(self):
        return self.counter


@t
def single_thread():
    task = SumTask()
    task.add_from(START, END)
    return task.get_sum()


@t
def multi_thread():
    task1 = SumTask()
    task2 = SumTask()
    thread1 = Thread(target=task1.add_from, args=(START, MID))
    thread2 = Thread(target=task2.add_from, args=(MID + 1, END))

    thread1.start()
    thread2.start()

    thread1.join()
    thread2.join()

    return task1.get_sum() + task2.get_sum()


def single_process(task: SumTask, start: int, end: int):
    task.add_from(start, end)


@t
def multi_processes():
    BaseManager.register("SumTask", SumTask)
    manager = BaseManager(address=("127.0.0.1", 50000), authkey=b"abc")
    manager.start()

    task1 = manager.SumTask()
    task2 = manager.SumTask()

    p1 = Process(target=single_process, args=(task1, START, MID))
    p2 = Process(target=single_process, args=(task2, MID + 1, END))

    p1.start()
    p2.start()

    p1.join()
    p2.join()

    result = task1.get_sum() + task2.get_sum()
    manager.shutdown()
    return result
