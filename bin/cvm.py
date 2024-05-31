from random import shuffle, random
from typing import Hashable, Self


class CVM:
    size: int
    seen: set
    prob: float

    def __init__(self: Self, size: int = 1024):
        self.size = size
        self.seen = set()
        self.prob = 1.0

    def _halve(self: Self):
        seen = list(self.seen)
        shuffle(seen)
        self.seen = set(seen[: int(len(seen) / 2)])
        self.prob /= 2

    def count_item(self: Self, item: Hashable) -> Self:
        if random() >= self.prob:
            self.seen.discard(item)
        elif item not in self.seen:
            self.seen.add(item)
            if len(self.seen) >= self.size:
                self._halve()
        return self

    @property
    def cardinality(self: Self) -> int:
        return len(self.seen) / self.prob


cvm = CVM(size=16384)
for i in range(50_000_000):
    cvm.count_item(i)
print(f"python: {cvm.cardinality}")

cvm2 = CVM(size=16384)
for i in range(50_000_000):
    cvm2.count_item(i % 1000)
print(f"python: {cvm2.cardinality}")
