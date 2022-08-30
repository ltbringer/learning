from typing import List

import pygame
import numpy as np

from natsim.animal import Animal
from natsim.plant import Plant


def main():
    width = 1850
    height = 1010
    screen = pygame.display.set_mode([width, height])

    animals: List[Animal] = pygame.sprite.Group()
    plants: List[Plant] = pygame.sprite.Group()

    for _ in range(100):
        Plant(max_x=width, max_y=height, groups=[plants])

    for _ in range(5):
        Animal(max_x=width, max_y=height, groups=[animals])

    running = True
    day = True
    ticks = 0
    while running:

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        ticks += 1

        if ticks % 1000 == 0:
            day = not day

        if day:
            screen.fill((5, 26, 54))
        else:
            screen.fill((0, 0, 0))

        for plant in plants:
            screen.blit(plant.surf, plant.position)
            if day:
                plant.grow()

        for animal in animals:
            screen.blit(animal.surf, animal.position)
            screen.blit(animal.eye.surf, animal.eye.position)

        for animal in animals:
            if animal.exhausted():
                animal.color = (0, 0, 0)
                animal.kill()
                continue

            animal.move(width, height)

            seen_plants = pygame.sprite.spritecollide(animal.eye, plants, dokill=False)
            animal.target(seen_plants)

            touched_plants = pygame.sprite.spritecollide(animal, plants, dokill=False)
            animal.digest(touched_plants)

        pygame.display.flip()
    pygame.quit()
