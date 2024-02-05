package ui

import (
	raygui "github.com/gen2brain/raylib-go/raygui"
	rl "github.com/gen2brain/raylib-go/raylib"
)

type MainMenuItem uint8

const (
	MainMenuItem_None MainMenuItem = iota
	MainMenuItem_NewGame
	MainMenuItem_LoadGame
	MainMenuItem_Options
	MainMenuItem_Quit
)

// MainMenu can be called in game loop to display the main menu
func MainMenu() MainMenuItem {
	item := MainMenuItem_None
	r := rl.NewRectangle(float32(rl.GetScreenWidth())/2-100, float32(rl.GetScreenHeight())/2-50, 200, 100)
	if raygui.Button(r, "New Game") {
		item = MainMenuItem_NewGame
	}
	r.Y += 100
	if raygui.Button(r, "Load Game") {
		item = MainMenuItem_LoadGame
	}
	r.Y += 100
	if raygui.Button(r, "Options") {
		item = MainMenuItem_Options
	}
	r.Y += 100
	if raygui.Button(r, "Quit") {
		item = MainMenuItem_Quit
	}
	return item
}
