# Makefile

SOURCES  := mecab2.rc mecab2.rs
LIB_DEPS := $(shell mecab-config --libs-only-L)

.PHONY: all
all: libmecab

libmecab: $(SOURCES)
	rustc -O mecab2.rc -L $(LIB_DEPS)

.PHONY: test
test: $(SOURCES)
	rustc -O mecab2.rc -L $(LIB_DEPS) --test

.PHONY: clean
clean:
	rm -r *.dSYM
