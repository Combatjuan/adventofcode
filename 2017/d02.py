#!/usr/bin/env python3
data = 'd02.txt'
sheet = []
with open(data) as f:
	for line in f:
		sheet.append([int(n) for n in line.split()])

answer = 0
for row in sheet:
	answer += max(row) - min(row)

print("Part 1:")
print(answer)


answer = 0
for row in sheet:
	row_n = None
	# O(N^2) is entirely feasible for small N
	for i, n in enumerate(row):
		if row_n is not None:break

		for m in row[i+1:]:
			if m % n == 0:
				print("{} / {} == {}".format(m, n, m // n))
				row_n = m // n
				break
			elif n % m == 0:
				print("{} / {} == {}".format(n, m, n // m))
				row_n = n // m
				break
	answer += row_n

print("Part 2:")
print(answer)



