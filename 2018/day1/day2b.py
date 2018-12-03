#!/usr/bin/env python
import sys

total = 0
remember = set()
changes = []

with open("input.txt") as the_file:
    for line in the_file:
        line = line.strip()
        symbol = line[0]
        number = int(line[1:])
        changes.append((symbol, number))

while True:
    for symbol, number in changes:
        remember.add(total)

        if symbol == "+":
            total = total + number
        elif symbol == "-":
            total = total - number
        else:
            print "WHAT ARE YOU TALKING ABOUT??? YOU SAID IT WOULD BE + OR -"
            sys.exit(1)

        print total

        if total in remember:
            print "Hey, I have seen ", total, "before!"
            sys.exit(0)

