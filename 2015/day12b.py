#!/usr/bin/env python
import json
import sys

sum = 0
with open(sys.argv[1], 'r') as f:
	j = json.load(f)

def just_values(value):
	val_list = []
	if isinstance(value, dict):
		for k, v in value.items():
			if v == 'red':
				return [0]
			val_list.extend(just_values(v))
	elif isinstance(value, list):
		for v in value:
			val_list.extend(just_values(v))
	else:
		val_list = [value]
	return val_list

d = dict(j)
for v in just_values(d):
	try:
		sum = sum + int(v)
	except:
		pass

print sum

