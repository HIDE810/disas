TARGET		:=	disas
SOURCE		:=	source
INCLUDE		:=	include

CPPFILES	=	$(wildcard $(SOURCE)/*.cpp)

all: $(TARGET)

clean:
	@$(RM) $(TARGET)

$(TARGET): $(CPPFILES)
	@$(CXX) -g -I $(INCLUDE) $^ -o $@