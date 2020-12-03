#ifndef MAP_H
#define MAP_H

#include <QFile>
#include <QTextStream>
#include <QVector>
#include <unit.h>

// =============================================================================
enum class TileType
{
	OPEN,
	ELF_CORPSE,
	GOBLIN_CORPSE,
	WALL
};

// =============================================================================
struct Tile
{
	Tile() : type(TileType::OPEN), unit(nullptr) {}
	Tile(TileType type) : type(type), unit(nullptr) {}
	TileType type;
	Unit* unit;

	bool HasTeam(Team team) const;
	bool IsWalkable() const;
};

// =============================================================================
class CheckMap
{
public:
	CheckMap();
	CheckMap(int width, int height);
	virtual ~CheckMap() { delete[] _checks; }
	bool Checked(const RowCol& rc) const { return _checks[rc.row * _width + rc.col] != RowCol::Null; }
	RowCol Previous(const RowCol& rc) const { return _checks[rc.row * _width + rc.col]; }
	void Walk(const RowCol& from, const RowCol& to) { _checks[to.row * _width + to.col] = from; }
	RowCol FirstStep(const RowCol& pos) const;

	int Width() const { return _width; }
	int Height() const { return _height; }

private:
	RowCol* _checks;
	int _width;
	int _height;
};

// =============================================================================
class Map
{
public:
	Map();
	virtual ~Map();

	void Load(const QString& filename);
	int elves;
	int goblins;
	QList<Unit*> units;

	int Width() const { return _width; }
	int Height() const { return _height; }

private:
	int _width;
	int _height;
	QVector<QVector<Tile>> _rows;
	RowCol _FillCheck(RowCol rc, QQueue<RowCol>& tiles, CheckMap& visited, Team enemy_type, bool halt);
	QList<Unit*> _the_dead;

public:
	void MoveUnit(RowCol from, RowCol to);
	void DestroyUnit(Unit* unit);
	void Cull();
	void Print();
	//void Print(const CheckMap& visited, const QQueue<RowCol>& fill_queue);
	bool StepTowardEnemy(const Unit& u, RowCol* nextStep);
	RowCol StepTowardEnemy(const Unit& u);

	Tile& At(int row, int col) { return _rows[row][col]; }
	Tile& At(const RowCol& pos) { return _rows[pos.row][pos.col]; }
	Unit* AdjacentUnit(const RowCol& rc, Team unit_type);
	Unit* AdjacentEnemy(const Unit& unit);
};

#endif // MAP_H
