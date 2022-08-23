import uuid
import colorsys
from typing import Tuple, Optional

import numpy as np
from pygame.sprite import Sprite, Group
from pygame import Surface


def random_vec(
    start: float, 
    end: float, 
    size: Optional[int] = None
) -> np.ndarray:
    if size:
        return np.random.uniform(start, end, size)
    return np.random.uniform(start, end)


def random_color() -> Tuple[int, int, int]:
    hue = np.random.uniform(0.2, 0.5)
    sat = np.random.uniform(0.42, 1)
    light = np.random.uniform(0.7, 1)
    return tuple(int(255 * x) for x in colorsys.hsv_to_rgb(hue, sat, light))


class Plant(Sprite):
    def __init__(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
        max_x: Optional[float] = None,
        max_y: Optional[float] = None,
        max_size: Optional[int] = None,
        color: Optional[Tuple[int, int, int]] = None,
        groups: Optional[Group] = None,
    ):
        super().__init__(*groups)
        self.id_ = uuid.uuid4()
        self.age = 0
        self.x = x or random_vec(0, max_x)
        self.y = y or random_vec(0, max_y)
        self.growth_rate = 0.1
        self.energy = 1
        self.size = 1
        self.max_size = max_size or np.random.choice([2, 5, 10, 50, 100], 1, p=[0.2, 0.3, 0.2, 0.1, 0.2])[0]
        self.color = color or random_color()
        self.surf = Surface((self.size, self.size))
        self.surf.fill(self.color)
        self.rect = self.surf.get_rect(center=(self.x, self.y))

        nutrients = ["attack", "defense", "speed", "vision"]
        n_nutrients = np.random.randint(0, len(nutrients))
        self.nutrients = {nutrient: np.random.uniform(0, 0.01) for nutrient in np.random.choice(nutrients, n_nutrients)}

    @classmethod
    def from_dict(cls, data: dict):
        return cls(**data)

    @property
    def radius(self):
        return self.size

    @property
    def position(self):
        return (self.x, self.y)

    def grow(self):
        if self.dead():
            return

        self.energy = min(self.energy + self.growth_rate, 1e6)
        for nutrient in self.nutrients:
            self.nutrients[nutrient] += self.growth_rate
        self.set_size()

    def set_size(self):
        self.size = min(np.sqrt(self.energy), self.max_size)
        self.surf = Surface((self.size, self.size))
        self.surf.fill(self.color)

    def diminish(self, absorbed_energy):
        self.energy = max(self.energy-absorbed_energy, 1)
        self.set_size()

    def alive(self):
        return self.energy > 0

    def dead(self):
        return not self.alive()
