#!/usr/bin/python3
from pprint import pprint

def main():
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

    solar_system = {}
    for (moon, planet) in orbits.items():
        solar_system.setdefault(moon, []).append(planet)
        solar_system.setdefault(planet, []).append(moon)

    transfers = dijkstra(solar_system, orbits['YOU'], orbits['SAN'])
    
    print('Transfers required: ', transfers)

def dijkstra(graph, start, finish):
    distances = { id:None for id in graph.keys() }
    distances[start] = 0

    visited = set()

    current_node = start

    while current_node != finish:
        visited.add(current_node)
        neighbours = [id for id in graph[current_node] if id not in visited]
        neighbour_distance = distances[current_node] + 1
        for neighbour in neighbours:
            if distances[neighbour] is None or distances[neighbour] > neighbour_distance:
                distances[neighbour] = neighbour_distance

        known_distances = { id:distance for (id, distance) in distances.items() if id not in visited and distance is not None }
        current_node = min(known_distances, key = known_distances.get)   

    return distances[finish]

if __name__ == "__main__":
    main()
