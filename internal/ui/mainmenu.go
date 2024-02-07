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
	width := 50 * vmin
	height := 10 * vmin
	r := rl.NewRectangle(vw*50-width/2, vh*50-height/2, width, height)
	raygui.SetFont(FontDefault)
	if raygui.Button(r, "New Game") {
		item = MainMenuItem_NewGame
	}
	r.Y += r.Height + vh
	if raygui.Button(r, "Load Game") {
		item = MainMenuItem_LoadGame
	}
	r.Y += r.Height + vh
	if raygui.Button(r, "Options") {
		item = MainMenuItem_Options
	}
	r.Y += r.Height + vh
	if raygui.Button(r, "Quit") {
		item = MainMenuItem_Quit
	}
	return item
}
