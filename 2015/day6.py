#!/usr/bin/env python
import sys

defs = {}

with open(sys.argv[1], 'r') as f:
	for line in f:
		parts = line.split()
		target = parts[-1]
		definition = '(' + ' '.join(parts[:-2]) + ')'
		definition.replace('AND', '&')
		definition.replace('OR', '|')
		definition.replace('LSHIFT', '<<')
		definition.replace('RSHIFT', '>>')
		definition.replace('NOT ', '~')

		defs[target] = definition
	print defs

