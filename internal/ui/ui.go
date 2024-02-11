package ui

import (
	rl "github.com/gen2brain/raylib-go/raylib"
)

var (
	FontDefault rl.Font
)

func Init() {
	FontDefault = rl.LoadFontEx("resources/fonts/alagard.ttf", 128, nil)
	updateUnits()
}
