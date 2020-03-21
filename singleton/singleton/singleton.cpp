#include <iostream>
#include <thread>

#include "ChocolateBoiler.h"

using std::thread;

int main()
{
	thread threads[5];

	ChocolateBoiler::instance().fill();

	for (auto& th : threads)
	{
		th = thread([] {
			auto& boiler = ChocolateBoiler::instance();
			boiler.boil();
		});
	}

	for (auto& th : threads)
	{
		th.join();
	}


	ChocolateBoiler::instance().drain();

	return 0;
}