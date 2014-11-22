OUT_DIR ?= .
SRC=lzf_c.c lzf_d.c
OBJ=$(SRC:%.c=$(OUT_DIR)/%.o)
HEADERS=lzf.h lzfP.h
LIB=liblzf.a

ARCH := $(word 1, $(subst -, ,$(TARGET)))
ifeq ($(ARCH),i686)
	CFLAGS += -m32 -fPIC
else
	CFLAGS += -m64 -fPIC
endif

all: $(LIB)

$(LIB): $(OBJ)
	$(AR) crus $(OUT_DIR)/$(LIB) $(OBJ)
	
$(OUT_DIR)/%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<
