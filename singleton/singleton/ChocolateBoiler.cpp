#include <iostream>

#include "ChocolateBoiler.h"

using std::lock_guard;
using std::mutex;
using std::cout;
using std::endl;


void ChocolateBoiler::fill()
{
	lock_guard<mutex> lock(mtx);
	if (is_empty)
	{
		cout << "fill" << endl;
		is_empty = false;
		is_boiled = false;
	}
	else
	{
		cout << "can't fill" << endl;
	}
}

void ChocolateBoiler::drain()
{
	lock_guard<mutex> lock(mtx);
	if (!is_empty && is_boiled)
	{
		cout << "drain" << endl;
		is_empty = true;
	}
	else
	{
		cout << "can't drain" << endl;
	}
}

void ChocolateBoiler::boil()
{
	lock_guard<mutex> lock(mtx);
	if (!is_empty && !is_boiled)
	{
		cout << "boil" << endl;
		is_boiled = true;
	}
	else
	{
		cout << "can't boil" << endl;
	}
}