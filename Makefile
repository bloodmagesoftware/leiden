PKG := leiden

ENTRYPOINT := ./cmd/$(PKG)

ifeq ($(OS),Windows_NT)
    SRCS := $(wildcard $(shell dir /b /s *.go 2>nul | find /v ".git" | find /v "vendor"))
	BINARY := $(PKG).exe
	FLAGS := -ldflags "-H windowsgui"
else
    SRCS := $(wildcard $(shell find . -name '*.go' -not -path './vendor/*'))
	BINARY := $(PKG)
	FLAGS := -ldflags "-s -w"
endif

build:
	go build -o $(BINARY) $(FLAGS) $(ENTRYPOINT)

run:
	go run $(ENTRYPOINT)

clean:
	rm -f $(BINARY)
