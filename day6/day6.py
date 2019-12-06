#!/usr/bin/python3

with open('puzzle_input.txt') as input:
    orbits = { moon:planet for (planet, moon) in [line.rstrip().split(')') for line in input] }

total_orbits = 0

for moon in orbits.keys():
    total_orbits += 1
    planet = orbits[moon]
    while planet != "COM":
        total_orbits += 1
        planet = orbits[planet]

print('Total orbits:', total_orbits)
