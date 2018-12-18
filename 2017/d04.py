#!/usr/bin/env python3
from collections import Counter
data = 'd04.txt'
valid = 0
with open(data) as f:
	for line in f:
		words = line.strip().split()
		if len(words) == len(set(words)):
			valid += 1
print("Part 1:")
print(valid)
		
valid = 0
with open(data) as f:
	for line in f:
		counts = [tuple(sorted(dict((Counter(x))))) for x in line.strip().split()]
		if len(counts) == len(set(counts)):
			valid += 1
print("Part 2:")
print(valid)

