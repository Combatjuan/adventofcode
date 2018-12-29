#include <iostream>
#include <vector>

using std::endl;
using std::cout;
using std::vector;

//const char* INPUT = "\x03\x02\x00\x08\x05";
//const int INPUT_LEN = 5;

//const char* INPUT = "\x03\x02\x00\x08\x05\x01";
//const char* INPUT = "\x1\x04\x07\x00\x06\x01";
const char* INPUT = "\x6\x03\x07\x00\x06\x01";
const int INPUT_LEN = 6;

//const char* INPUT = "\x08\x09";
//const int INPUT_LEN = 2;

bool matches(const vector<unsigned char>& scores)
{
	int start = scores.size() - INPUT_LEN;
	for (int i = 0; i < INPUT_LEN; ++i)
	{
		if (INPUT[i] != scores[start + i]) return false;
	}
	return true;
}

int main(int argc, char* argv[])
{
	int elf1 = 0;
	int elf2 = 1;
	vector<unsigned char> scores;
	scores.push_back(3);
	scores.push_back(7);
	scores.reserve(10000000);

	int i = 0;
	for (; ; ++i)
	{
		unsigned char next = scores[elf1] + scores[elf2];
		if (next > 9)
		{
			scores.push_back(next / 10);
			scores.push_back(next - 10);
		}
		else scores.push_back(next);
		elf1 = (elf1 + 1 + scores[elf1]) % scores.size();
		elf2 = (elf2 + 1 + scores[elf2]) % scores.size();

		if (matches(scores)) break;

		//if (scores.size() % 10000000 == 0) cout << scores.size() << endl;
	}

//!	for (vector<unsigned char>::iterator it = scores.begin(); it != scores.end(); ++it)
//!	{
//!		cout << " " << (int)*it;
//!	}
	cout << i << endl;

	return 0;
}
 
