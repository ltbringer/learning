from __future__ import annotations
import uuid
import colorsys
from typing import Callable, Tuple, Optional

import numpy as np
import pygame
from pygame.sprite import Sprite, Group
from pygame import Surface
from natsim.plant import Plant


def random_vec(
    start: float, 
    end: float, 
    size: Optional[int] = None
) -> Callable[[], np.ndarray]:
    if size:
        return np.random.uniform(start, end, size)
    return np.random.uniform(start, end)


def random_color() -> Tuple[int, int, int]:
    hue = np.random.uniform(0, 1)
    sat = np.random.uniform(0.5, 1)
    light = np.random.uniform(0.7, 1)
    return tuple(int(255 * x) for x in colorsys.hsv_to_rgb(hue, sat, light))


class Eye(Sprite):
    def __init__(
        self,
        size: int,
        x: float,
        y: float
    ):
        super().__init__()
        self.x = x
        self.y = y
        self.surf = Surface((size, size), pygame.SRCALPHA)
        self.rect = self.surf.get_rect(center=(x, y))

    @property
    def position(self):
        return (self.x, self.y)


class Animal(Sprite):
    def __init__(
        self,
        x: Optional[float] = None,
        y: Optional[float] = None,
        max_x: Optional[float] = None,
        max_y: Optional[float] = None,
        vector: Optional[np.ndarray] = None,
        color: Optional[Tuple[int, int, int]] = None,
        groups: Optional[Group] = None,
    ):
        super().__init__(*groups)
        self.id_ = uuid.uuid4()
        self.age = 0
        self.x = x or random_vec(0, max_x)
        self.y = y or random_vec(0, max_y)
        self.vector = vector or random_vec(-1, 1, 2)

        self.energy = 10
        self.size = 5

        self.stats = {
            "attack": {"value": 0, "max": 20},
            "defense": {"value": 0, "max": 20},
            "speed": {"value": 0.5, "max": 1},
            "vision": {"value": 500, "max": 500},
        }

        self.color = (255, 255, 255)
        self.surf = Surface((self.size, self.size))
        self.rect = self.surf.get_rect(center=(self.x, self.y))
        self.surf.fill(self.color)
        self.eye = Eye(self.stats["vision"]["value"], self.x, self.y)

    @classmethod
    def from_dict(cls, data: dict):
        return cls(**data)

    @property
    def fitness(self):
        return self.energy + self.stats["attack"] + self.stats["defense"]

    @property
    def speed(self):
        return self.stats["speed"]["value"]

    @property
    def vision(self):
        return self.stats["vision"]["value"]

    @property
    def radius(self):
        return self.size

    @property
    def position(self):
        return np.array([self.x, self.y])

    def active(self):
        return self.energy > 0

    def exhausted(self):
        return not self.alive()

    def target(self, plants):
        plants_ = sorted(plants, key=lambda plant: plant.energy)
        for plant in plants_:
            plant: Plant
            if plant.energy > 10:
                magnitude = np.linalg.norm(plant.position - self.position)
                self.vector = (plant.position - self.position) / magnitude

    def update_position(self, max_size, value):
        if value > max_size:
            return value - max_size

        if value < 0:
            return max_size + value

        return value

    def move(self, width: int, height: int):
        if self.exhausted():
            return
        dx = self.vector[0] * self.speed
        dy = self.vector[1] * self.speed
        self.x = self.update_position(width, self.x + dx)
        self.y = self.update_position(height, self.y + dy)
        self.energy -= 1
        self.rect = self.surf.get_rect(center=(self.x, self.y))
        self.eye.rect = self.eye.surf.get_rect(center=(self.x, self.y))
        if self.energy < 100:
            self.color = (255, 0, 0)

    def digest(self, plants: Plant):
        for plant in plants:
            absorbed_energy = max(plant.energy - 1, 0)
            plant.diminish(absorbed_energy)
            self.energy += absorbed_energy
            if absorbed_energy > 1000:
                print(f"Absorbed {absorbed_energy} from plant {plant.id_}, it now has {plant.energy} left.")
            if self.energy > 100:
                self.color = (255, 255, 255)

            for nutrient in plant.nutrients:
                new_stat = plant.nutrients[nutrient] + self.stats[nutrient]["value"]
                self.stats[nutrient]["value"] = min(new_stat, self.stats[nutrient]["max"])
