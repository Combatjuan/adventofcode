#!/usr/bin/env python
import ctypes
import sys

WIDTH = 0
HEIGHT = 0

grid = []
with open(sys.argv[1], 'r') as f:
	for line in f:
		grid.append(line.strip())
		HEIGHT += 1
		WIDTH = len(line.strip())

def make_grid():
	return [ctypes.create_string_buffer('.' * WIDTH)] * HEIGHT

def neighbors(g, x, y):
	n = 0
		
	def exists(x, y):
		return x >=0 and y >= 0 and x < WIDTH and y < WIDTH
	
	if exists(x - 1, y - 1) and g[y-1][x-1] == '#': n += 1
	if exists(x - 1, y + 0) and g[y][x-1] == '#': n += 1
	if exists(x - 1, y + 1) and g[y+1][x-1] == '#': n += 1
	if exists(x + 0, y - 1) and g[y][x] == '#': n += 1
	if exists(x + 0, y + 1) and g[y+1][x] == '#': n += 1
	if exists(x + 1, y - 1) and g[y-1][x+1] == '#': n += 1
	if exists(x + 1, y + 0) and g[y][x+1] == '#': n += 1
	if exists(x + 1, y + 1) and g[y+1][x+1] == '#': n += 1

	return n

def next_grid(g):
	next = make_grid()
	for y, row in enumerate(g):
		for x, col in enumerate(row):
			if neighbors(g, x, y) in (2, 3):
				next[y][x] = '#'
	print_grid(next)
	return next

def print_grid(g):
	for row in grid:
		print str(row)
		#for c in row:
			#print c,

print "GRID IS", WIDTH, "x", HEIGHT
for i in range(100):
	grid = next_grid(grid)

count = 0
for y, row in enumerate(grid):
	for x, col in enumerate(row):
		if grid[y][x] == '#':
			count += 1

print count
