import time
import functools
from concurrency.utils.logger import logger


def t(fn):
    @functools.wraps(fn)
    def wrapper(*args, **kwargs):
        start = time.process_time()
        result = fn(*args, **kwargs)
        end = time.process_time()
        logger.debug(
            f"\n▫️{fn.__name__} took {(end - start):.3f} seconds -- returns {result}."
        )
        return result

    return wrapper
