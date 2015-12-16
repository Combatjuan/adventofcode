#!/usr/bin/env python
from collections import namedtuple
import itertools
import re
import sys

Ingredient = namedtuple('Ingredient', ['name', 'capacity', 'durability', 'flavor', 'texture', 'calories'])

ingredients = {}
cookie_re = re.compile('^(\w*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)')
with open(sys.argv[1], 'r') as f:
	for line in f:
		m = cookie_re.match(line.strip())
		if m:
			ingredient = Ingredient(*m.groups())
			ingredient = Ingredient(m.groups()[0],
				int(m.groups()[1]),
				int(m.groups()[2]),
				int(m.groups()[3]),
				int(m.groups()[4]),
				int(m.groups()[5]),
			)
			ingredients[ingredient.name] = ingredient
		else:
			print "Could not match line: '%s'" % line.strip()


def score(recipe, display=False):
	capacity = 0
	durability = 0
	flavor = 0
	texture = 0
	calories = 0
	for ingredient, quantity in recipe.items():
		capacity += ingredients[ingredient].capacity * quantity
		durability += ingredients[ingredient].durability * quantity
		flavor += ingredients[ingredient].flavor * quantity
		texture += ingredients[ingredient].texture * quantity
		calories += ingredients[ingredient].calories * quantity
	if capacity < 0:
		capacity = 0
	if durability < 0:
		durability = 0
	if flavor < 0:
		flavor = 0
	if texture < 0:
		texture = 0
	if calories < 0:
		calories = 0

	if calories == 500:
		if display:
			print "  capacity: %d, durability: %d, flavor: %d, texture: %d" % (capacity, durability, flavor, texture)
		return (capacity * durability * flavor * texture)
	else:
		return 0
	
print ingredients
print

max = 0
for combos in itertools.combinations_with_replacement(ingredients.keys(), 100):
	recipe = {}
	for i in combos:
		recipe.setdefault(i, 0)
		recipe[i] += 1
	s = score(recipe)
	if s > max:
		max = s
		print recipe, s
		score(recipe, True)


