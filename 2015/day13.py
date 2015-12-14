#!/usr/bin/env python
import re
import sys
import itertools

happies = {}

with open(sys.argv[1], 'r') as f:
	for line in f:
		m = re.search('^(\w*) would (gain|lose) (\d+) happiness units by sitting next to (\w*)\\.', line)
		if m:
			name = m.groups()[0]
			number = int(m.groups()[2]) if m.groups()[1] == 'gain' else -int(m.groups()[2])
			toward = m.groups()[3]
			if name in happies:
				happies[name][toward] = number
			else:
				happies[name] = {toward: number}
		else:
			print "No match for", line

def calc_happies(order, print_vals=False):
	total = 0
	for i, name in enumerate(order[:-1]):
		a = name
		b = order[i + 1]
		total += happies[a][b] + happies[b][a]
	a = order[-1]
	b = order[0]
	total += happies[a][b] + happies[b][a]
	return total

def me_too():
	for k in list(happies.keys()):
		happies.setdefault('Myself', {})[k] = 0
		happies[k]['Myself'] = 0

# For part B.
me_too()

# Because I can never remember the name of a bigint constant and am lazy.
max = 0
perms = itertools.permutations(happies.keys())
optimal = None

for p in perms:
	happy_factor = calc_happies(p)
	if happy_factor > max:
		max = happy_factor
		optimal = p

print optimal
calc_happies(optimal, True)
print max

