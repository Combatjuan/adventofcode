#!/usr/bin/env python

count = 0
lines = []
with file('day3.input') as f:
	for line in f:
		line = line.strip()
		lines.append([int(n) for n in line.split()])

for n in range(0, len(lines), 3):
	a = sorted([lines[n][0], lines[n+1][0], lines[n+2][0]])
	b = sorted([lines[n][1], lines[n+1][1], lines[n+2][1]])
	c = sorted([lines[n][2], lines[n+1][2], lines[n+2][2]])
	print n
	print a, b, c
	count = count + (1 if (a[0] + a[1] > a[2]) else 0)
	count = count + (1 if (b[0] + b[1] > b[2]) else 0)
	count = count + (1 if (c[0] + c[1] > c[2]) else 0)

print count
