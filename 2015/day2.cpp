#include <fstream>
#include <iostream>
#include <string>

using std::ifstream;
using std::string;
using std::endl;
using std::cout;

int min_side(int l, int w, int h)
{
	int a = l * w;
	int b = l * h;
	int c = w * h;
	if (a <= b && a <= c) return a;
	else if (b <= a && b <= c) return b;
	else return c;
}

int main(int argc, char* argv[])
{
	if (argc != 2)
	{
		cout << "Invalid arguments.  Provide one filename." << endl;
		return 1;
	}
	string filename(argv[0]);
	ifstream f;
	f.open(filename.c_str(), std::ifstream::in);
	int loop = 0;
	int total = 0;
	int subtotal = 0;
	if (f.is_open())
	{
		int l, w, h;
		char x1, x2;
		while (!f.eof())
		{
			f >> l >> x1 >> w >> x2 >> h;
			cout << l << endl;
			cout << w << endl;
			cout << h << endl;
			subtotal = (2 * l * w) + (2 * l * h) + (2 * w * h) + min_side(l, w, h);
			cout << subtotal;
			total = total + subtotal;
			if (loop > 5) {
				cout << "Something is dumb.  Aborting." << endl;
				return 3;
			}
			loop++;
		}
	}
	else
	{
		cout << "Error opening file." << endl;
		return 2;
	}
	cout << total << endl;
	return 0;
}

