#!/usr/bin/env python
import sys

with open(sys.argv[1], 'r') as f:
	s = f.read()
	houses = {(0, 0): 1}
	sx = sy = rx = ry = 0
	n = 0
	for c in s:
		if n % 2:
			if c == '^':
				sy += 1
			elif c == 'v':
				sy -= 1
			elif c == '>':
				sx += 1
			elif c == '<':
				sx -= 1

			tup = (sx, sy)
		else:
			if c == '^':
				ry += 1
			elif c == 'v':
				ry -= 1
			elif c == '>':
				rx += 1
			elif c == '<':
				rx -= 1

			tup = (rx, ry)

		if tup in houses:
			houses[tup] += 1
		else:
			houses[tup] = 1
		n = n + 1
	print len(houses)
