#!/usr/bin/env python

with file('day2.input') as f: lines = f.readlines()

x = 1
y = 3
keypad = [
		[None,	None,	None,	None,	None, 	None,	None],
		[None,	None,	None,	1,		None, 	None,	None],
		[None,	None,	2,		3,		4,	 	None,	None],
		[None,	5,		6,		7,		8,	 	9,		None],
		[None,	None,	'A',	'B',	'C', 	None,	None],
		[None,	None,	None,	'D',	None, 	None,	None],
		[None,	None,	None,	None,	None, 	None,	None],
		[1, 2, 3], [4, 5, 6], [7, 8, 9]
]

def move(dx, dy):
	global x, y
	if keypad[y + dy][x + dx] is None: return
	x, y = x + dx, y + dy

code = ''
for line in lines:
	for char in line:
		if char == 'U': move(0, -1)
		if char == 'D': move(0, 1)
		if char == 'L': move(-1, 0)
		if char == 'R': move(1, 0)
	print keypad[y][x]

