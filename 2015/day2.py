#!/usr/bin/env python
import sys

def smallest_side(l, w, h):
	a = l * w
	b = w * h
	c = l * h
	if a <= b and a <= c:
		return a
	elif b <= a and b <= c:
		return b
	else:
		return c
	
def calc_ribbon(l, w, h):
	parts = sorted([l, w, h])[:2]
	x, y = parts[0], parts[1]
	return 2 * x + 2 * y + l * w * h

with open(sys.argv[1], 'r') as f:
	total = 0
	ribbon = 0
	for line in f:
		parts = line.split('x')
		l, w, h = int(parts[0]), int(parts[1]), int(parts[2])
		subtotal = 2 * l * w + 2 * w * h + 2 * h * l + smallest_side(l, w, h)
		total += subtotal
		ribbon += calc_ribbon(l, w, h)
	print total
	print ribbon

