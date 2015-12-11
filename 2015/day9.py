#!/usr/bin/env python
import sys
import itertools

cities = {}

with open(sys.argv[1], 'r') as f:
	for line in f:
		parts = line.split()
		a = parts[0]
		b = parts[2]
		d = int(parts[4])

		cities.setdefault(a, {})
		cities.setdefault(b, {})

		cities[a][b] = d
		cities[b][a] = d

for k, v in cities.items():
	print k, v
print
print

min = 100000
max = 0
	
def distance(trip):
	froms = trip[:-1]
	tos = trip[1:]
	d = 0
	for x, y in zip(froms, tos):
		d += cities[x][y]
	return d

for trip in itertools.permutations(list(cities)):
	trip_length = distance(trip)
	if trip_length < min:
		min = trip_length
		shortest = trip
	if trip_length > max:
		max = trip_length
		longest = trip
print "Longest"
print longest
print max
print

print "Shortest"
print shortest
print min

