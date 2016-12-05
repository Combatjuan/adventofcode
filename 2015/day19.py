#!/usr/bin/env python
import re
import string
import sys

replacement_line = re.compile('(\w+)\s*=>\s*(\w+)')

replacements = {}
start = None

with open(sys.argv[1], 'r') as f:
	for line in f:
		m = replacement_line.search(line)
		if m:
			replacements.setdefault(m.groups()[0], []).append(m.groups()[1])
		elif line.strip():
			start = line.strip()

possibilities = set()

for r, chems in replacements.items():
	next = 0
	while next != -1:
		next = string.find(start, r, next)
		if next != -1:
			for c in chems:
				new = start[:next] + c + start[next+len(r):]
				print r, c, new
				possibilities.add(new)

print len(possibilities)

