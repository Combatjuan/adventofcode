#!/usr/bin/env python
import sys

sizes = []
with open(sys.argv[1], 'r') as f:
	for line in f:
		sizes.append(int(line.strip()))

sizes = list(reversed(sorted(sizes)))

def fits(amount, containers):
	if amount == 0:
		yield []
	else:
		original_numbers = list(containers)
		for n in original_numbers:
			n = containers[0]
			containers = containers[1:]
			for sub_fit in fits(amount - n, containers):
				yield [n] + sub_fit

print "Part A"
combos = list(fits(150, sizes))
print len(combos)

m = min([len(l) for l in combos])
print "Part B"
print len([l for l in combos if len(l) == m])

