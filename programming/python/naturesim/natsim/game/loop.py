from typing import List

import pygame
from natsim.animal import Animal
from natsim.plant import Plant


def main():
    width = 1900
    height = 1080
    screen = pygame.display.set_mode([width, height])

    animals: List[Animal] = pygame.sprite.Group()
    plants: List[Plant] = pygame.sprite.Group()
    all_sprites = pygame.sprite.Group()

    for _ in range(2000):
        Plant(max_x=width, max_y=height, groups=[plants, all_sprites])

    for _ in range(1):
        Animal(max_x=width, max_y=height, groups=[animals, all_sprites])

    running = True
    while running:

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        screen.fill((0, 0, 0))

        for sprite in all_sprites:
            screen.blit(sprite.surf, sprite.position)

        for plant in plants:
            plant.grow()

        for animal in animals:
            if animal.exhausted():
                animal.color = (0, 0, 0)
                animal.kill()
                continue

            animal.move(width, height)
            touched_plants = pygame.sprite.spritecollide(animal, plants, dokill=False)
            for plant in touched_plants:
                animal.digest(plant)

        # Flip the display
        pygame.display.flip()

    # Done! Time to quit.
    pygame.quit()
