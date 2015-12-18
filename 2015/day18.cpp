#include <iostream>
#include <string>
#include <sstream>
#include <fstream>

using namespace std;
const int WIDTH = 100;
const int HEIGHT = 100;
const int ITERATIONS = 100;

class Grid
{
public:
	Grid(int w, int h) : _width(w), _height(h) {
		data = new char[w * h];
	};
	~Grid() {
		delete data;
	}

	void next() {
		char* c = new char[_width * _height];
		for (int y = 0; y < _height; ++y)
		{
			for (int x = 0; x < _width; ++x)
			{
				int count = 0;
				count += neighbors(x - 1, y - 1);
				count += neighbors(x + 0, y - 1);
				count += neighbors(x + 1, y - 1);
				count += neighbors(x - 1, y + 0);
				count += neighbors(x + 1, y + 0);
				count += neighbors(x - 1, y + 1);
				count += neighbors(x + 0, y + 1);
				count += neighbors(x + 1, y + 1);

				if (
					x == 0 && y == 0 ||
					x == 0 && y == _height - 1 ||
					x == _width - 1 && y == 0 ||
					x == _width - 1 && y == _height -1)
				{
					c[y * _width + x] = '#';
				}
				else if (count == 2 && data[y * _width + x] == '#') c[y * _width + x] = '#';
				//if (count == 2 && data[y * _width + x] == '#') c[y * _width + x] = '#';
				else if (count == 3) c[y * _width + x] = '#';
				else c[y * _width + x] = '.';
			}
		}
		delete data;
		data = c;
	}

	void print() {
		for (int y = 0; y < _height; ++y)
		{
			for (int x = 0; x < _width; ++x)
			{
				cout << data[y * _width + x];
			}
			cout << endl;
		}
	}

	int count() {
		int count = 0;
		for (int y = 0; y < _height; ++y)
		{
			for (int x = 0; x < _width; ++x)
			{
				if (data[y * _width + x] == '#') ++count;
			}
		}
		return count;
	}

	char* data;

private:
	int neighbors(int x, int y) {
		if (x >= 0 and y >= 0 and x < _width and y < _height && data[y * _width + x] == '#') return 1;
		else return 0;
	}

	int _width;
	int _height;
};

int main(int argc, char* argv[])
{
	ifstream f;
	f.open(argv[1]);
	Grid g(WIDTH, HEIGHT);
	if (f.is_open())
	{
		string line;
		int n = 0;
		while (std::getline(f, line))
		{
			cout << line << endl;
			for (string::const_iterator i = line.begin(); i != line.end(); ++i)
			{
				g.data[n] = *i;
				++n;
			}
		}
	}

	for (int i = 0; i < ITERATIONS; ++i)
	{
		g.print();
		cout << endl;
		g.next();
	}

	cout << "Count:" << endl;
	cout << g.count() << endl;

	return 0;
}

