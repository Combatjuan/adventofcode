#include "map.h"
#include "color.h"
#include <cassert>
#include <iostream>
#include <QTextStream>
#include <QThread>
#include <QQueue>

// =============================================================================
bool Tile::HasTeam(Team team) const
{
	return unit && unit->team == team;
}

bool Tile::IsWalkable() const
{
	return !unit && type != TileType::WALL;
}

// =============================================================================
CheckMap::CheckMap(int width, int height)
	: _width(width), _height(height)
{
	_checks = new RowCol[width * height];
	for (int i = 0; i < width * height; ++i)
	{
		_checks[i] = RowCol::Null;
	}
}

RowCol CheckMap::FirstStep(const RowCol& pos) const
{
	RowCol step = RowCol(pos);
	while (Previous(Previous(step)) != RowCol::Null)
	{
		step = Previous(step);
	}
	return step;
}

// =============================================================================
Map::Map() :
	elves(0),
	goblins(0),
	_width(0),
	_height(0)
{

}

Map::~Map()
{
	for (auto punit : units) delete punit;
}

Unit* Map::AdjacentUnit(const RowCol& rc, Team unit_type)
{
	QList<Unit*> adjacents;
	if (rc.row > 0 && At(rc.above()).HasTeam(unit_type))
	{
		adjacents.push_back(At(rc.above()).unit);
	}
	if (rc.col > 0 && At(rc.left()).HasTeam(unit_type))
	{
		adjacents.push_back(At(rc.left()).unit);
	}
	if (rc.col < _width - 1 && At(rc.right()).HasTeam(unit_type))
	{
		adjacents.push_back(At(rc.right()).unit);
	}
	if (rc.row < _height - 1 && At(rc.below()).HasTeam(unit_type))
	{
		adjacents.push_back(At(rc.below()).unit);
	}

	std::sort(adjacents.begin(), adjacents.end(), [](auto a, auto b) { return a->hp < b->hp || (a->hp == b->hp && a->pos < b->pos); } );
	if (!adjacents.empty()) return adjacents.first();
	else return nullptr;
}

Unit* Map::AdjacentEnemy(const Unit& unit)
{
	Team enemy_type = (unit.team == Team::ELF) ? Team::GOBLIN : Team::ELF;
	return AdjacentUnit(unit.pos, enemy_type);
//!	QList<Unit*> adjacents;
//!
//!	if (At(unit.pos.above()).HasTeam(enemy_type)) adjacents.push_back(At(unit.pos.above()).unit);
//!	if (At(unit.pos.left()).HasTeam(enemy_type)) adjacents.push_back(At(unit.pos.left()).unit);
//!	if (At(unit.pos.right()).HasTeam(enemy_type)) adjacents.push_back(At(unit.pos.right()).unit);
//!	if (At(unit.pos.below()).HasTeam(enemy_type)) adjacents.push_back(At(unit.pos.below()).unit);
//!
//!	std::sort(adjacents.begin(), adjacents.end(), [](auto a, auto b) { return a->hp < b->hp || a->hp == b->hp && a->pos < b->pos; } );
//!	if (!adjacents.empty()) return adjacents.first();
//!	else return nullptr;
}

// Check to see if there is an enemy adjacent
// If so, return that spot.
// Else append adjacent open spots to the walk list.
RowCol Map::_FillCheck(RowCol rc, QQueue<RowCol>& fill_queue, CheckMap& visited, Team enemy_type, bool halt)
{
	RowCol adjacents[4] = {rc.above(), rc.left(), rc.right(), rc.below()};
	for (RowCol pos : adjacents)
	{
		if (!visited.Checked(pos))
		{
			const Tile& t = At(pos);
			Unit* adjacent_enemy = AdjacentUnit(pos, enemy_type);
			if (t.IsWalkable() && adjacent_enemy)
			{
				visited.Walk(rc, pos);
				return pos;
			}
			else if (!halt && t.IsWalkable())
			{
				visited.Walk(rc, pos);
				fill_queue.enqueue(pos);
			}
		}
	}
	return RowCol::Null;
}

RowCol Map::StepTowardEnemy(const Unit& u)
{
	CheckMap visited(_width, _height);
	QQueue<RowCol> fill_queue;
	QList<RowCol> closest;
	Team enemy_type = u.team == Team::ELF ? Team::GOBLIN : Team::ELF;

	visited.Walk(RowCol::Null, u.pos);
	fill_queue.enqueue(u.pos);
	while (!fill_queue.empty())
	{
		RowCol rc = fill_queue.dequeue();
		RowCol next_step = _FillCheck(rc, fill_queue, visited, enemy_type, !closest.empty());
		if ((bool)next_step)
		{
			RowCol a(-1, -1);
			RowCol b(4, 3);
			closest.push_back(next_step);
		}
	}

	// If we found at least one enemy
	if (!closest.empty())
	{
		std::sort(closest.begin(), closest.end());
		RowCol first_step = visited.FirstStep(closest.first());
		return first_step;
	}
	else return RowCol::Null;
}

void Map::MoveUnit(RowCol from, RowCol to)
{
	Tile& from_t = At(from);
	Tile& to_t = At(to);

	int delta = from.row - to.row + from.col - to.col;
	assert(from_t.unit != nullptr);
	assert(to_t.unit == nullptr);
	assert(to_t.type != TileType::WALL);
	assert(delta == -1 || delta == 1);
	
	Unit* u = from_t.unit;
	u->pos = to;
	to_t.unit = u;
	from_t.unit = nullptr;
}

void Map::DestroyUnit(Unit* unit)
{
	//QTextStream out(stdout);
	//out << "Here falls " << *unit << endl;

	Tile& t = At(unit->pos);
	t.unit = nullptr;
	if (unit->team == Team::ELF)
	{
		t.type = TileType::ELF_CORPSE;
		--elves;
	}
	else
	{
		t.type = TileType::GOBLIN_CORPSE;
		--goblins;
	}
	int i = units.indexOf(unit);
	_the_dead.push_back(unit);
	units.removeAt(i);
}

void Map::Cull()
{
	for (auto unit : _the_dead) delete unit;
	_the_dead.clear();
}

void Map::Load(const QString& filename)
{
	QFile f;
	f.setFileName(filename);
	if (f.open(QIODevice::ReadOnly))
	{
		QTextStream data(&f);
		QTextStream out(stdout);
		int row = 0;
		int width = -1;
		while (!f.atEnd())
		{
			QString line = f.readLine().trimmed();
			QVector<Tile> tiles;
			int col = 0;
			for (QChar qc : line)
			{
				char c = qc.toLatin1();
				switch (c) {
				case '#':
					tiles.push_back(Tile(TileType::WALL));
					break;
				case '.':
					tiles.push_back(Tile(TileType::OPEN));
					break;
				case 'E':
					units.push_back(new Unit(RowCol(row, col), Team::ELF));
					tiles.push_back(Tile(TileType::OPEN));
					tiles.last().unit = units.last();
					this->elves++;
					break;
				case 'G':
					units.push_back(new Unit(RowCol(row, col), Team::GOBLIN));
					tiles.push_back(Tile(TileType::OPEN));
					tiles.last().unit = units.last();
					this->goblins++;
					break;
				default:
					out << "Error parsing map.  Unexpected " << c << " at " << row << ", " << col << endl;
				}
				if (width < 0) width = line.size();
				else assert(line.size() == width);
				++col;
			}
			_rows.push_back(tiles);
			_height = _rows.size();
			_width = width;
			out << line << endl;
			++row;
		}

		for (auto unit : this->units)
		{
			out << "UNIT: " << *unit << "---" << unit->pos << endl;
		}
	}
}

void Map::Print()
{
	QTextStream out(stdout);
	//out << "\033[2J";
	//out << "\033[3J";
	for (auto row : _rows)
	{
		for (const Tile& t : row)
		{
			if (t.HasTeam(Team::ELF)) out << GREEN << 'E' << RESET;
			else if (t.HasTeam(Team::GOBLIN)) out << RED << 'G' << RESET;
			else if (t.type == TileType::ELF_CORPSE) out << GREEN << '%' << RESET;
			else if (t.type == TileType::GOBLIN_CORPSE) out << RED << '%' << RESET;
			else if (t.IsWalkable()) out << '.';
			else out << '#';
		}
		out << endl;
	}
}

/*
void Map::Print(const CheckMap& visited, const QQueue<RowCol>& fill_queue)
{
	QTextStream out(stdout);
	out << "Map: " << Height() << "x" << Width() << " - Check " << visited.Height() << "x" << visited.Width() << endl;

	//out << "\033[2J";
	//out << "\033[3J";
	for (int r = 0; r < _height; ++r)
	{
		for (int c = 0; c < _width; ++ c)
		{
			const Tile& t = At(r, c);
			if (!fill_queue.empty() && r == fill_queue.first().row && c == fill_queue.first().col) out << "?";
			else if (t.HasTeam(Team::ELF)) out << GREEN << 'E' << RESET;
			else if (t.HasTeam(Team::GOBLIN)) out << RED << 'G' << RESET;
			else if (!t.IsWalkable()) out << ((c % 5 == 0 && r % 5 == 0) ? '+' : '#');
			else if (visited.Checked(RowCol(r, c))) out << BBLACK << "@" << RESET;
			else out << ((c % 5 == 0 && r % 5 == 0) ? ',' : '.');
		}
		out << endl;
	}
	for (auto rc : fill_queue) { out << rc << ", "; }
	out << endl;
}
*/

