#ifndef CHOCOLATE_BOILER_H
#define CHOCOLATE_BOILER_H

#include <mutex>

class ChocolateBoiler
{
	bool is_empty = true;
	bool is_boiled = false;
	std::mutex mtx;

private:
	ChocolateBoiler() = default;
	~ChocolateBoiler() = default;

public:
	ChocolateBoiler(const ChocolateBoiler&) = delete;
	ChocolateBoiler& operator=(const ChocolateBoiler&) = delete;

	void fill();
	void drain();
	void boil();

public:
	static ChocolateBoiler& instance()
	{
		static ChocolateBoiler boiler; // thread-safe
		return boiler;
	}
};


#endif