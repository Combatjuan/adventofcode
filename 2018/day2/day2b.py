#!/usr/bin/env python3

DEBUG = False
def debug_print(s):
	print(s)

sets = {}

def add_to_sets(sets, s):
	for i in range(0, len(s) - 1):
		t = s[0:i] + s[i+1:]
		if t in sets.get(i, set()):
			return t
		else:
			sets.setdefault(i, set()).add(t)

with open("input.txt") as the_file:
	for line in the_file:
		s = line.strip()
		duplicate = add_to_sets(sets, s)
		if duplicate:
			print (duplicate, "(" + s + ")")

