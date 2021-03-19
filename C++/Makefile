TARGET		:=	disas
SOURCE		:=	source
INCLUDE		:=	include
CPPFILES	:=	$(wildcard $(SOURCE)/*.cpp)

ifeq ($(OS),Windows_NT)
	EXT	:=	.exe
else
	EXT	:=	.out
endif

all: $(TARGET)

clean:
	@$(RM) $(TARGET)$(EXT)

re: clean all

$(TARGET): $(CPPFILES)
	@$(CXX) -g -I $(INCLUDE) $^ -o $@$(EXT)
