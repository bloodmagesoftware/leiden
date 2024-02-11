package main

import (
	"github.com/bloodmagesoftware/leiden/internal/ui"
	"github.com/bloodmagesoftware/leiden/internal/window"
	rl "github.com/gen2brain/raylib-go/raylib"
)

func main() {
	if err := window.Init(); err != nil {
		panic(err)
	}
	defer window.Close()

	ui.Init()

	ui.RaylibIntro()
	ui.BloodmageIntro()

game_loop:
	for window.Open() {
		ui.Update()
		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)
		i := ui.MainMenu()
		switch i {
		case ui.MainMenuItem_Quit:
			break game_loop
		}
		rl.EndDrawing()
	}
}
