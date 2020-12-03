#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

using namespace std;

int fuel_required(int mass)
{
	return (mass / 3) - 2;
}

int main(int argc, char* argv[])
{
	if (argc != 2)
	{
		cerr << "Usage: cmd <input_file>" << endl;
		return 1;
	}

	ifstream f(argv[1]);
	string line;
	stringstream ss;
	int fuel = 0;
	while (std::getline(f, line))
	{
		ss << line;

		int mass;
		ss >> mass;
		int item_fuel = fuel_required(mass);
		cout << "mass: " << mass << "	fuel: " << item_fuel << "	running: " << fuel << endl ;
		fuel += item_fuel;

		ss.clear();
	}
	cout << fuel << endl;
	f.close();

	return 0;
}
