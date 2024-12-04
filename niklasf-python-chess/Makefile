all: .venv

.venv:
	virtualenv .venv
	.venv/bin/pip install -r requirements.txt

clean:
	rm -Rf .venv

run: .venv
	ls -l simul.py
	.venv/bin/python simul.py 1000
