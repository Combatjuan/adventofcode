#!/usr/bin/env python
import sys

def toggleX(grid, x1, y1, x2, y2):
	new = []
	for r in range(SIZE):
		row = grid[r]
		if r >= y1 and r <= y2:
			left = row[:x1]
			right = row[x2+1:]
			f = lambda x: x + 2
			middle = [f(c) for c in row[x1:x2+1]]
			new.append(left + middle + right)
		else:
			new.append(row)
	return new
	
def onX(grid, x1, y1, x2, y2):
	new = []
	for r in range(SIZE):
		row = grid[r]
		if r >= y1 and r <= y2:
			left = row[:x1]
			right = row[x2+1:]
			f = lambda x: x + 1
			middle = [f(c) for c in row[x1:x2+1]]
			new.append(left + middle + right)
		else:
			new.append(row)
	return new
	
def offX(grid, x1, y1, x2, y2):
	new = []
	for r in range(SIZE):
		row = grid[r]
		if r >= y1 and r <= y2:
			left = row[:x1]
			right = row[x2+1:]
			f = lambda x: max(x - 1, 0)
			middle = [f(c) for c in row[x1:x2+1]]
			new.append(left + middle + right)
		else:
			new.append(row)
	return new
	
#!def draw(grid):
#!	for row in grid:
#!		print row
#!	print

def sum(grid):
	n = 0
	for row in grid:
		for col in row:
			n = n + col
	return n

#draw(grid)
with open(sys.argv[1], 'r') as f:
	SIZE = 1000
	row = [0 for x in range(SIZE)]
	grid = [row for i in range(SIZE)]

	for line in f:
		#print
		print line
		words = line.split()
		x1, y1 = 0, 0
		x2, y2 = 0, 0
		if line.startswith('turn on'):
			line_parts = words[2].split(',')
			x1, y1 = int(line_parts[0]), int(line_parts[1])
			line_parts = words[4].split(',')
			x2, y2 = int(line_parts[0]), int(line_parts[1])
			grid = onX(grid, x1, y1, x2, y2)

		elif line.startswith('turn off'):
			line_parts = words[2].split(',')
			x1, y1 = int(line_parts[0]), int(line_parts[1])
			line_parts = words[4].split(',')
			x2, y2 = int(line_parts[0]), int(line_parts[1])
			grid = offX(grid, x1, y1, x2, y2)

		elif line.startswith('toggle'):
			line_parts = words[1].split(',')
			x1, y1 = int(line_parts[0]), int(line_parts[1])
			line_parts = words[3].split(',')
			x2, y2 = int(line_parts[0]), int(line_parts[1])
			grid = toggleX(grid, x1, y1, x2, y2)

		else:
			raise RuntimeError("Could not understand command.")
	
		#draw(grid)
		#print sum(grid), len(grid), len(grid[0])

	print sum(grid)

