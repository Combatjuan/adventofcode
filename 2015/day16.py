#!/usr/bin/env python
import sys

# Read and parse input into a dictionary of dictionaries.
sues = {}
with open(sys.argv[1], 'r') as f:
	for line in f:
		name_keys = line.split(':', 1)
		number = int(name_keys[0].split()[1])
		key_vals = name_keys[1].split(',')
		d = {}
		for kv in key_vals:
			object, quantity = kv.split(':')
			d[object.strip()] = int(quantity.strip())
		sues[number] = d

detected = {
	'children': 3,
	'cats': 7,
	'samoyeds': 2,
	'pomeranians': 3,
	'akitas': 0,
	'vizslas': 0,
	'goldfish': 5,
	'trees': 3,
	'cars': 2,
	'perfumes': 1,
}

print "Detected:", detected

for num, remembered in sues.items():
	matches_a = True
	matches_b = True
	for object, quantity in detected.items():
		if object in remembered:
			if quantity != remembered[object]:
				matches_a = False
			if object in ('cats', 'trees'):
				if quantity >= remembered[object]:
					matches_b = False
			elif object in ('pomeranians', 'goldfish'):
				if quantity <= remembered[object]:
					matches_b = False
 			else:
				if quantity != remembered[object]:
					matches_b = False
	if matches_a:
		print "Part A: Could be #%d: %s" % (num, remembered)
	if matches_b:
		print "Part B: Could be #%d: %s" % (num, remembered)

