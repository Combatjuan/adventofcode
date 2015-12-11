#!/usr/bin/env python
import sys

with open(sys.argv[1], 'r') as f:
	s = f.read()
	houses = {(0, 0): 1}
	x = 0
	y = 0
	n = 0
	for c in s:
		if c == '^':
			y += 1
		elif c == 'v':
			y -= 1
		elif c == '>':
			x += 1
		elif c == '<':
			x -= 1

		tup = (x, y)
		if tup in houses:
			houses[tup] += 1
		else:
			houses[tup] = 1
		n = n + 1
	print len(houses)
