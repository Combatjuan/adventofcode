#!/usr/bin/env python
import re


def inc_char(c):
	if c == 'z': return 'a'
	else: return chr(ord(c) + 1)
	
def tri_increments(password):
	ords = [ord(c) for c in password]
	for i, o in enumerate(ords[:-2]):
		if o + 2 ==  ords[i+1] + 1 == ords[i+2]:
			return True
	return False

dbl_dbls = re.compile('(.)\\1.*(.)\\2')
has_oil = re.compile('[oil]')

def matches(password):
	return has_oil.search(password) is None and dbl_dbls.search(password) is not None and tri_increments(password)


password = 'hxbxwxba'
while True:
	# I think this may actually be the fastest implementation even though it's ugly...
	print password
	incs = 0
	password = password[:7] + inc_char(password[7])
	if password[6] == 'z':
		incs += 1
		if password[5] == 'z':
			incs += 1
			if password[4] == 'z':
				incs += 1
				if password[3] == 'z':
					incs += 1
					if password[2] == 'z':
						incs += 1
						if password[1] == 'z':
							incs += 1
							if password[0] == 'z':
								incs += 1
	password = password[:8-incs] + ''.join([inc_char(c) for c in password[incs:]])

	if matches(password):
		break

print password
