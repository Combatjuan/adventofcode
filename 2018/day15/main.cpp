#include <QCoreApplication>
#include <QThread>
#include <QTextStream>
#include <cassert>
#include <map.h>

using std::endl;

/*
template <typename T>
struct PtrLess
{     
	bool operator()(const T* a, const T* b) const     
	{
		return *a < *b;
	} 
}; 
*/

int main(int argc, char *argv[])
{
	QCoreApplication a(argc, argv);

	QTextStream out(stdout);

	QString filename = "input.txt";
	if (argc > 1) filename = argv[1];
	out << "Loading " << filename << "..." << endl;
	Map map;
	map.Load(filename);
	out << "Map loaded." << endl;

	int round = 0;
	while (map.elves > 0 && map.goblins > 0)
	{
		//out << "Round: " << round << endl;
		std::sort(map.units.begin(), map.units.end(),
				[](const Unit* a, const Unit* b) -> bool { return a->pos < b-> pos; });

		QList<Unit*> units = map.units;
		for (auto unit : units)
		{
			if (!unit->IsAlive()) continue;
			if (map.elves == 0 || map.goblins == 0) goto done;

			Unit* enemy = map.AdjacentEnemy(*unit);
			if (enemy)
			{
				enemy->Hit();
				if (!enemy->IsAlive()) map.DestroyUnit(enemy);
			}
			else
			{
				RowCol rc = map.StepTowardEnemy(*unit);
				if (!rc.IsNull())
				{
					map.MoveUnit(unit->pos, rc);
					enemy = map.AdjacentEnemy(*unit);
					if (enemy)
					{
						enemy->Hit();
						if (!enemy->IsAlive())
						{
							map.DestroyUnit(enemy);
						}
					}
				}
			}
		}
		map.Print();
		map.Cull();
		QThread::msleep(100);
		++round;
	}
done:
	map.Print();
	out << "Done" << endl;

	int hp = 0;
	Team winner = (map.goblins > 0) ? Team::GOBLIN : Team::ELF;
	for (auto survivor : map.units)
	{
		assert(survivor->team == winner);
		hp += survivor->hp;
	}
	out << "After " << round << " rounds there are " << map.goblins << " survivors left with " << hp << "HP = " << round * hp << endl;

	return 0;
}

