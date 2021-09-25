import time


def t(fn):
    def wrapper(*args, **kwargs):
        start = time.process_time()
        result = fn(*args, **kwargs)
        end = time.process_time()
        print(
            f"\n▫️{fn.__name__} took {(end - start):.3f} seconds -- returns {result}."
        )
        return result

    return wrapper
