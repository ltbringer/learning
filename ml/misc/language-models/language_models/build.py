import random
from typing import Tuple, Union, Iterator, List, Optional
from collections import defaultdict

from language_models.preprocess import preprocess_line


def get_file_lines(file_name: str):
    with open(file_name, "r") as f:
        for line in f.read().splitlines():
            yield line


def preprocess_lines(lines: Iterator[str]) -> Iterator[str]:
    for line in lines:
        yield preprocess_line(line)


def make_ngrams(
    lines: Iterator[str], n=1, start="#", end="#"
) -> Iterator[Tuple[str, ...]]:
    for line in lines:
        ngrams = []
        line = f"{start}{start}{line}{end}"
        for i in range(len(line)):
            if i + n > len(line):
                break
            ngrams.append(line[i : i + n])
        yield tuple(ngrams)


class LanguageModel:
    def __init__(
        self,
        file_name: str,
        ngrams: int = 1,
        start: str = "#",
        end: str = "#",
        random_state: int = 0,
    ) -> None:
        self.file_name = file_name
        self.ngrams = ngrams
        self.ngram_counts: defaultdict = defaultdict(lambda: defaultdict(lambda: 0))
        self.ngram_proba: defaultdict = defaultdict(lambda: defaultdict(lambda: 0.0))
        self.start = start
        self.end = end
        self.random_state = random_state

    def __build_ngram_counts(self):
        lines = preprocess_lines(get_file_lines(self.file_name))
        for ngram_list in make_ngrams(
            lines, n=self.ngrams, start=self.start, end=self.end
        ):
            for c1, c2, c3 in ngram_list:
                self.ngram_counts[(c1, c2)][c3] += 1
        return self

    def __build_ngram_proba(self):
        for given in self.ngram_counts:
            total = sum(self.ngram_counts[given].values())
            for char in self.ngram_counts[given]:
                self.ngram_proba[given][char] = self.ngram_counts[given][char] / total
        return self

    def fit(self):
        self.__build_ngram_counts()
        self.__build_ngram_proba()
        return self

    def validate(self, given: Union[Tuple[str, ...], str] = None) -> None:
        if not isinstance(given, tuple):
            raise Exception(f"{given=} must be a tuple or a string")
        if given not in self.ngram_proba:
            raise Exception(f"{given=} not found in the language model")


    def p(self, char: str, given: Union[Tuple[str, ...], str] = None) -> float:
        if isinstance(given, str):
            given = tuple(given)
        self.validate(given)
        return self.ngram_proba[given][char]

    def n_best(self, given: Union[Tuple[str, ...], str] = None, n=1) -> List[str]:
        if isinstance(given, str):
            given = tuple(given)
        self.validate(given)
        return sorted(self.ngram_proba[given], key=lambda x: self.ngram_proba[given][x])[:n]

    def generate_single(self, seed: str):
        sentence = seed
        while self.end != sentence[-1]:
            new_seed_pos = -self.ngrams + 1
            options = self.n_best(given=sentence[new_seed_pos:], n=3)
            next_char = random.choice(options)
            sentence += next_char
        return sentence.replace(self.start, "").replace(self.end, "")

    def generate(self, seed: Optional[str] = None, count=1, fn=None) -> str:
        if seed is None:
            seed = "".join(random.choice(list(self.ngram_proba.keys())))
        sentences = "\n".join([self.generate_single(seed) for _ in range(count)])
        return fn(sentences) if fn else sentences
