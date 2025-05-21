OS := $(shell uname)
ifeq ($(OS), Darwin)
CFG_FOR_DARWIN=-DDARWIN
else
CFG_FOR_DARWIN=-UDARWIN
endif

PREFIX=se205-lab3
TEACHER=true
CC=gcc
CFLAGS=-g -Wall -DTEACHER=$(TEACHER) $(CFG_FOR_DARWIN)
LDFLAGS=-g -DTEACHER=$(TEACHER) -pthread

PRESOURCES_1=\
main_blocking_queue.c\
blocking_queue.c\
blocking_queue.h\
cond_blocking_queue.c\
cond_blocking_queue.h\
sem_blocking_queue.c\
sem_blocking_queue.h\
utils.c\
utils.h\

SOURCES_1 = \
bounded_buffer.h\
bounded_buffer.c\

OBJECTS_1 = \
bounded_buffer.o\
main_blocking_queue.o\
blocking_queue.o\
cond_blocking_queue.o\
sem_blocking_queue.o\
utils.o\

PRESOURCES = \
$(PRESOURCES_1)\

SOURCES = \
$(SOURCES_1)\

OBJECTS = \
$(OBJECTS_1)\

PROGS = \
main_blocking_queue\

%.c: %.p.c
	awk -f presources.awk -v TEACHER=$(TEACHER) $< >$@

%.h: %.p.h
	awk -f presources.awk -v TEACHER=$(TEACHER) $< >$@

.c.o:
	$(CC) -c $(CFLAGS) $<

default : $(PROGS)

clean : 
	$(RM) $(OBJECTS) $(PROGS) deps *~

veryclean: clean
	$(RM) $(PRESOURCES)

main_blocking_queue : $(PRESOURCES_1) $(OBJECTS_1)
	$(CC) $(LDFLAGS) -o $@ $(OBJECTS_1) 

deps: $(SOURCES) $(PRESOURCES)
	$(CC) -M $(SOURCES) $(PRESOURCES) >deps

student:
	@make veryclean
	@make TEACHER=false $(PRESOURCES)

teacher:
	@make veryclean
	@make TEACHER=true $(PRESOURCES)

index.html: index.texi
	makeinfo \
	        --no-headers --html --ifinfo --no-split \
		--css-include=style.css $< > $@

error :
	$(error "PREFIX variable not set")

install : veryclean index.html
	@if test -z "$(PREFIX)"; then \
	   make error; \
	fi
	@make student
	-mkdir -p $(PREFIX)
	chmod og=u-w $(PREFIX)
	tar zcf src.tar.gz `cat MANIFEST` Makefile
	chmod og=u-w style.css index.html src.tar.gz
	cp -p style.css index.html src.tar.gz $(PREFIX)

-include deps
