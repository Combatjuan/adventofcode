#!/usr/bin/env python3
from math import ceil, sqrt
import sys
data = 'd02.txt'
sheet = []

def manhattan_spiral(n):
	distance = 0
	if n <= 1: return 0

	# Derived via Triangular Numbers Partial Sum
	ring = ceil((sqrt(n) -1) / 2.0)
	# Calculating the start of the ring
	ring_start = 8 * ((ring - 1) * ring // 2) + 2
	# The offset is how far we are around the ring.
	offset = n - ring_start
	# Each ring has a pattern that repeats 4 times (1 per side) and is len(ring * 2).
	# How far we are along a side is a simple modulus.
	pattern_offset = offset % (ring * 2)
	# If we're past the middle of the pattern, we're further away
	if pattern_offset >= ring:
		distance = pattern_offset + 1
	# If we're before the middle of the pattern, we're further away in the other direction.
	else:
		distance = (2 * ring) - pattern_offset - 1
	return distance

data = 289326

print("Part 1:")
print(manhattan_spiral(data))

# Part 2:
# #$*()#$%()*#$%*()#$%*()#$% And now I do have to actually spacially map it for part 2.
# So much for O(1) solutions
print("Part 2:")
class Spiraler(object):
	def __init__(self):
		self.x, self.y = 0, 0
		self.step = 0
		self.side = 1
		self.horizontal = True
		self.positive = True
	
	def __iter__(self):
		return self

	def __next__(self):
		return self.next()

	def next(self):
		lastx, lasty = self.x, self.y
		self.step += 1
		if self.horizontal:
			self.x += (1 if self.positive else -1)
		else:
			self.y += (1 if self.positive else -1)

		if self.step >= self.side:
			self.step = 0
			if not self.horizontal:
				self.positive = not self.positive
				self.side += 1
			self.horizontal = not self.horizontal
		return lastx, lasty

def get_adjacent(x, y, filled):
	if x == 0 and y == 0:
		return 1
	return (
			filled.get((x + 1, y + 1), 0) +
			filled.get((x + 1, y - 1), 0) +
			filled.get((x + 1, y + 0), 0) +
			filled.get((x + 0, y + 1), 0) +
			filled.get((x + 0, y - 1), 0) +
			filled.get((x - 1, y + 1), 0) +
			filled.get((x - 1, y + 0), 0) +
			filled.get((x - 1, y - 1), 0)
	)

spiraler = Spiraler()
filled = {(0, 0): 1}
for i, (x, y) in enumerate(spiraler):
	i += 1
	f = get_adjacent(x, y, filled)
	if f > data:
		print(f)
		sys.exit(0)
	filled[(x, y)] = f

