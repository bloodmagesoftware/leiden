package ui

import (
	"sync/atomic"
	"time"

	rl "github.com/gen2brain/raylib-go/raylib"
)

func BloodmageIntro() {
	bms := rl.LoadTexture("resources/textures/bms.png")
	rl.SetTextureFilter(bms, rl.FilterAnisotropic8x)
	active := atomic.Bool{}
	active.Store(true)
	title := "Bloodmage Software"
	go func() {
		<-time.After(5 * time.Second)
		active.Store(false)
	}()
	bloodRed := rl.NewColor(196, 27, 26, 255)
	for !rl.WindowShouldClose() && active.Load() {
		updateUnits()
		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)
		// draw centered with 1:1 aspect ratio
		imgS := 50 * vmin
		imgSrcRect := rl.Rectangle{X: 0, Y: 0, Width: float32(bms.Width), Height: float32(bms.Height)}
		imgDestRect := rl.Rectangle{X: 50*vw - imgS/2, Y: 30 * vh, Width: imgS, Height: imgS}
		rl.DrawTexturePro(bms, imgSrcRect, imgDestRect, rl.Vector2{X: 0, Y: 0}, 0, rl.White)

		// draw title
		fontS := 5 * vmin
		ts := rl.MeasureTextEx(FontDefault, title, fontS, 1)
		rl.DrawTextEx(FontDefault, title, rl.Vector2{X: 50*vw - ts.X/2, Y: 30*vh - ts.Y*2}, fontS, 1, bloodRed)
		rl.EndDrawing()
	}
	rl.UnloadTexture(bms)
}
