all: simul

simul: main.cpp chess.hpp
	g++ -std=c++17 -O3 -o simul main.cpp
	strip simul

clean:
	rm -f simul simul.gz

run: simul
	gzip -c simul > simul.gz
	ls -l simul simul.gz
	./simul 10000
