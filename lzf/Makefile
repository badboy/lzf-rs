CC=gcc
SRC=lzf_c.c lzf_d.c
OBJ=lzf_c.o lzf_d.o
HEADERS=lzf.h lzfP.h
LIB=liblzf.a

ARCH := $(word 1, $(subst -, ,$(TARGET)))
ifeq ($(ARCH),i686)
	CFLAGS += -m32 -fPIC
else
	CFLAGS += -m64 -fPIC
endif

all: $(OBJ)
	$(AR) crus $(OUT_DIR)/$(LIB) $(OBJ)
	
%.o: %.c
	$(CC) $(CFLAGS) -c -o $(OUT_DIR)/$@ $< 
