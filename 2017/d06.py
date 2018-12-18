#!/usr/bin/env python3

data = "10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6"
banks = [int(n) for n in data.split()]

def redistribute(banks):
	b = 0
	amount = 0
	for i, n in enumerate(banks):
		if n > amount:
			b = i
			amount = n
	
	banks[b] = 0
	while amount > 0:
		b += 1
		if b >= len(banks): b = 0
		amount -= 1
		banks[b] += 1

repeated = None
count = 0
history = set()
while True:
	t = tuple(banks)
	if t in history:
		print(banks)
		print("Part 1:")
		print(count)
		repeated = t
		break

	history.add(t)
	print(banks)
	redistribute(banks)
	count += 1

print()

count2 = 0
while True:
	count2 += 1
	redistribute(banks)
	t = tuple(banks)
	if t == repeated:
		print(banks)
		print("Part 2:")
		print(count2)
		break
	print(banks)

