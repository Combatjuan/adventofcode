#!/usr/bin/env python
import sys

with file('day1.input') as f:
	direction_string = f.read()

directions = direction_string.split(", ")
places = set((0, 0))
visited_twice = None

x = 0
y = 0

dx = 0
dy = 1


def rotate_left():
	global dx, dy
	dx, dy = dy, -dx

def rotate_right():
	global dx, dy
	dx, dy = -dy, dx

n = 0
for d in directions:
	
	turn = d[0]
	distance = int(d[1:])

	if turn in ('R', 'r'):
		rotate_right()
	elif turn in ('L', 'l'):
		rotate_left()
	else:
		print "Invalid directions:", d
		sys.exit(1)
	
	for i in range(distance):
		if visited_twice is None:
			if (x, y) in places:
				visited_twice = (x, y)
		places.add((x, y))
		x = x + dx
		y = y + dy

print '------'
print int(abs(x) + abs(y))
print int(abs(visited_twice[0]) + abs(visited_twice[1]))
