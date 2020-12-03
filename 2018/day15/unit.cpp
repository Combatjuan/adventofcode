#include "unit.h"

const int START_HP = 200;
const int DEFAULT_ATTACK = 45;
int Unit::_elf_id = 0;
int Unit::_goblin_id = 0;

const RowCol RowCol::Null = {-1, -1};

QTextStream& operator<<(QTextStream& out, const RowCol& rc)
{
	out << "(" << rc.row << ", " << rc.col << ")";
	return out;
}

Unit::Unit(const RowCol& row_col, Team team) :
	team(team),
	hp(START_HP),
	pos(row_col)
{
	if (team == Team::ELF) _id = _elf_id++;
	else _id = _goblin_id++;
}

QTextStream& operator<<(QTextStream& out, const Unit& unit)
{
	out << (unit.team == Team::ELF ? 'E' : 'G') << unit.id() << "[" << unit.hp << "]" << unit.pos;
	return out;
}

int Unit::Hit() { return Hit(DEFAULT_ATTACK); }

int Unit::Hit(int hit)
{
	hp -= hit;
	if (hp < 0) hp = 0;
	return hp;
}
