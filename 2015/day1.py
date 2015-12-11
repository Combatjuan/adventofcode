#!/usr/bin/env python
import sys

floor = 0
n = 0
entered = None
with open(sys.argv[1], 'r') as f:
	s = f.read()
	for c in s:
		n = n + 1
		if c == '(':
			floor = floor + 1
		elif c == ')':
			floor = floor - 1
		if floor < 0 and entered is None:
			entered = n
			print entered
	print floor

