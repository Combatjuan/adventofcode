#!/usr/bin/env python3
import re
import sys

LINE_DEFINITION = re.compile('#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9+]+)')
def parse_line(s):
	# Parses a line, returns id, x, y, w, h
	m = LINE_DEFINITION.match(s)
	if m:
		g = m.groups()
		return int(g[0]), int(g[1]), int(g[2]), int(g[3]), int(g[4])
	else:
		raise InputError("Non-matching line: '{}'".format(s))

rects = []
filename = "input.txt"
if len(sys.argv) > 1:
	filename = sys.argv[1]
with open(filename) as f:
	for line in f:
		s = line.strip()
		i, x, y, w, h = parse_line(s)
		rects.append((i, x, y, w, h))

width = max([x + w for _, x, _, w, _ in rects])
height = max([y + h for _, _, y, _, h in rects])
no_overlap = set([i for i, _, _, _, _ in rects])

fabric = [[0 for _ in range(width)] for _ in range(height)]

duplicates = 0
for (i, x, y, w, h) in rects:
	overlaps = False
	for c in range(x, x+w):
		for r in range(y, y+h):
			existing = fabric[r][c]
			if existing > 0:
				duplicates = duplicates + 1
				no_overlap.discard(existing)
				no_overlap.discard(i)
				fabric[r][c] = i
			elif existing == 0:
				fabric[r][c] = i

print(duplicates)
assert(len(no_overlap) == 1)
print(no_overlap.pop())

