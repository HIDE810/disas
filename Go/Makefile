TARGET	:=	disas

BUILD = go build

ifeq ($(OS),Windows_NT)
    EXT :=  .exe
else
    EXT :=  .out
endif

all: $(TARGET)$(EXT)

clean:
	@$(RM) $(TARGET)$(EXT)

re: clean all

$(TARGET)$(EXT):
	@$(BUILD) -o $@
