# Makefile

LIBDIR   := lib
TESTDIR  := test
SOURCES  := mecab.rc mecab.rs
LIB_DEPS := $(shell mecab-config --libs-only-L)

.PHONY: all
all: libmecab

libmecab: setup-lib $(SOURCES)
	rustc -O mecab.rc -L $(LIB_DEPS) --out-dir $(LIBDIR)

.PHONY: test
test: setup-test $(SOURCES)
	rustc -O mecab.rc -L $(LIB_DEPS) --test --out-dir $(TESTDIR)

setup-lib:
	mkdir -p $(LIBDIR)

setup-test:
	mkdir -p $(TESTDIR)

.PHONY: clean
clean:
	if [ -d "$(LIBDIR)" ]; then rm -r "$(LIBDIR)"; fi
	if [ -d "$(TESTDIR)" ]; then rm -r "$(TESTDIR)"; fi
