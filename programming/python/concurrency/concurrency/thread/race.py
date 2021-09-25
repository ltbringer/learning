import time
import random
from threading import Thread
from concurrency.utils import t


class RandomGen:
    def __init__(self, num, start=0, end=9):
        self.num = num
        self.start = start
        self.end = end
        self.random = 0
        self.value = 0

    def update(self):
        while True:
            self.random = random.randint(self.start, self.end)

    def get_multiple_of(self):
        while True:
            if self.random % self.num == 0:
                if self.random % self.num != 0:
                    self.value = self.random
                    break

@t
def eval_random_gen(num, start=0, end=9):
    random_gen = RandomGen(num, start=start, end=end)
    update_thread = Thread(target=random_gen.update, daemon=True)
    getter_thread = Thread(target=random_gen.get_multiple_of, daemon=True)

    update_thread.start()
    getter_thread.start()

    time.sleep(0.1)

    getter_thread.join()
    return random_gen.value
