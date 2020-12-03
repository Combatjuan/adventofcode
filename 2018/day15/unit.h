#ifndef UNIT_H
#define UNIT_H

#include <QTextStream>

enum class Team
{
	ELF,
	GOBLIN
};

struct RowCol
{
	RowCol() : row(-1), col(-1) {}
	RowCol(int row, int col) : row(row), col(col) {};
	operator bool() const { return !this->IsNull(); }
	bool operator==(const RowCol& other) const { return other.row == row && other.col == col; }
	bool operator!=(const RowCol& other) const { return ! (other.row == row && other.col == col); }
	bool operator<(const RowCol& other) const
	{
		return this->row < other.row || (this->row == other.row && this->col < other.col);
	}
	RowCol above() const { return RowCol(row - 1, col); }
	RowCol below() const { return RowCol(row + 1, col); }
	RowCol left() const { return RowCol(row, col- 1); }
	RowCol right() const { return RowCol(row, col + 1); }
	bool IsNull() const { return *this == RowCol::Null; }

	static const RowCol Null;

	int row;
	int col;
};

class Unit
{
public:
	Unit(const RowCol& row_col, Team team);
	bool operator<(const Unit& other) const { return this->pos < other.pos; }

	int Hit();
	int Hit(int hit);
	int id() const { return _id; }

	bool IsAlive() const { return hp > 0; }

	//QTextStream& operator<<(QTextStream& out);

	Team team;
	int hp;
	RowCol pos;

private:
	int _id;
	static int _elf_id;
	static int _goblin_id;
};

QTextStream& operator<<(QTextStream& out, const Unit& unit);
QTextStream& operator<<(QTextStream& out, const RowCol& rc);

#endif // UNIT_H
