package main

import (
	"github.com/bloodmagesoftware/leiden/internal/ui"
	"github.com/bloodmagesoftware/leiden/internal/window"
	"github.com/charmbracelet/log"
	rl "github.com/gen2brain/raylib-go/raylib"
)

func main() {
	if err := window.Init(); err != nil {
		panic(err)
	}
	defer window.Close()

	for window.Open() {
		rl.BeginDrawing()
		rl.ClearBackground(rl.RayWhite)
		log.Debug("MainMenu", "item", ui.MainMenu())
		rl.EndDrawing()
	}
}
