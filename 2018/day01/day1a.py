#!/usr/bin/env python
import sys

total = 0

with open("input.txt") as the_file:
    for line in the_file:
        line = line.strip()
        symbol = line[0]
        number = int(line[1:])
        #last_total = total

        if symbol == "+":
            total = total + number
        elif symbol == "-":
            total = total - number
        else:
            print "WHAT ARE YOU TALKING ABOUT??? YOU SAID IT WOULD BE + OR -"
            sys.exit(1)

        #print last_total, symbol, number, "=", total

print total
