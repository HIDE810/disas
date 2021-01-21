BUILD  = go build

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

all: clean $(TARGET)

clean:
	@$(RM) $(TARGET)

$(TARGET):
	@$(BUILD) -o $@
