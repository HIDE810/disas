SOURCE		:=	source
INCLUDE		:=	include
CPPFILES	:=	$(wildcard $(SOURCE)/*.cpp)

# Windows
ifeq ($(OS),Windows_NT)
	TARGET	:=	$(notdir $(CURDIR)).exe
endif

# Linux
ifeq ($(shell uname),Linux)
	TARGET	:=	$(notdir $(CURDIR)).out
endif

# Others
TARGET		?=	$(notdir $(CURDIR))

all: $(TARGET)

clean:
	@$(RM) $(TARGET)

re: clean all

$(TARGET): $(CPPFILES)
	@$(CXX) -g -I $(INCLUDE) $^ -o $@
