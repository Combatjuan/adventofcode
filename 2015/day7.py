#!/usr/bin/env python
import sys

defs = {}
evaluated = {}

with open(sys.argv[1], 'r') as f:
	for line in f:
		parts = line.split()
		target = parts[-1]
		definition = '( ' + ' '.join(parts[:-2]) + ' )'
		definition = definition.replace('AND', '&')
		definition = definition.replace('OR', '|')
		definition = definition.replace('LSHIFT', '<<')
		definition = definition.replace('RSHIFT', '>>')
		definition = definition.replace('NOT', '~')

		defs[target] = definition

# =======
STACK = []
def push(x):
	global STACK
	STACK.append(x)
def pop():
	global STACK
	STACK.pop()
for k, v in sorted(defs.items()):
	print "%s: %s" % (k, v)

def evaluate(s):
	global defs
	print STACK
	parts = s.split()
	new_parts = []
	for part in parts:
		if part in defs:
			if part in STACK:
				print STACK
				print "Circular reference detected."
				sys.exit(1)
			push(part)
			e = evaluate(defs[part])
			try:
				value = eval(e)
				defs[part] = str(value)
				part = str(value)
			except:
				part = e
			pop()
		new_parts.append(part)
	return ' '.join(new_parts)
		
e = evaluate('a')
print e

