
sources := mecab.rc mecab.rs

all: libmecab

libmecab: $(sources)
	rustc -O mecab.rc

test: $(sources)
	rustc -O mecab.rc --test

clean:
	rm -r *.dSYM
