package ui

import (
	"github.com/gen2brain/raylib-go/raygui"
	rl "github.com/gen2brain/raylib-go/raylib"
)

var (
	vw   float32
	vh   float32
	vmin float32
)

func updateUnits() {
	vw = float32(rl.GetScreenWidth()) / 100
	vh = float32(rl.GetScreenHeight()) / 100
	vmin = vh
}

func Update() {
	updateUnits()
	raygui.SetStyle(raygui.DEFAULT, raygui.TEXT_SIZE, int64(4*vmin))
	raygui.SetStyle(raygui.DEFAULT, raygui.BORDER_WIDTH, int64(vmin))
}
