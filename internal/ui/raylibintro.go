package ui

import (
	"time"

	rl "github.com/gen2brain/raylib-go/raylib"
)

func RaylibIntro() {
	// blinking
	{
		done := false
		i := 0
		go func() {
			for i = 0; i < 6; i++ {
				<-time.After(300 * time.Millisecond)
			}
			done = true
		}()
		for {
			if rl.WindowShouldClose() {
				return
			}
			if done {
				break
			}
			updateUnits()
			logoPositionX := int32(50*vw - 128)
			logoPositionY := int32(50*vh - 128)
			rl.BeginDrawing()
			rl.ClearBackground(rl.Black)
			if i&0x1 == 0 {
				rl.DrawRectangle(logoPositionX, logoPositionY, 16, 16, rl.RayWhite)
			}
			rl.EndDrawing()
		}
	}

	// top and left bars growing
	{
		done := false
		now := time.Now()
		duration := 500 * time.Millisecond
		go func() {
			<-time.After(duration)
			done = true
		}()
		for {
			if rl.WindowShouldClose() {
				return
			}
			if done {
				break
			}
			updateUnits()
			logoPositionX := int32(50*vw - 128)
			logoPositionY := int32(50*vh - 128)
			percent := float32(time.Since(now).Seconds()) / float32(duration.Seconds())
			rl.BeginDrawing()
			rl.ClearBackground(rl.Black)
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(16+240*percent), 16, rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY, 16, int32(16+240*percent), rl.RayWhite)
			rl.EndDrawing()
		}
	}

	// bottom and right bars growing
	{
		done := false
		now := time.Now()
		duration := 500 * time.Millisecond
		go func() {
			<-time.After(duration)
			done = true
		}()
		for {
			if rl.WindowShouldClose() {
				return
			}
			if done {
				break
			}
			updateUnits()
			logoPositionX := int32(50*vw - 128)
			logoPositionY := int32(50*vh - 128)
			percent := float32(time.Since(now).Seconds()) / float32(duration.Seconds())
			rl.BeginDrawing()
			rl.ClearBackground(rl.Black)
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(16+240), 16, rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY, 16, int32(16+240), rl.RayWhite)
			rl.DrawRectangle(logoPositionX+240, logoPositionY, 16, int32(16+240*percent), rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY+240, int32(16+240*percent), 16, rl.RayWhite)
			rl.EndDrawing()
		}
	}

	// letters appearing (one by one)
	{
		done := false
		title := "raylib"
		display := ""
		l := len(title)
		go func() {
			for i := 0; i <= l; i++ {
				<-time.After(200 * time.Millisecond)
				display = title[:i]
			}
			<-time.After(2 * time.Second)
			done = true
		}()
		for {
			if rl.WindowShouldClose() {
				return
			}
			if done {
				break
			}
			updateUnits()
			logoPositionX := int32(50*vw - 128)
			logoPositionY := int32(50*vh - 128)
			rl.BeginDrawing()
			rl.ClearBackground(rl.Black)
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(16+240), 16, rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY, 16, int32(16+240), rl.RayWhite)
			rl.DrawRectangle(logoPositionX+240, logoPositionY, 16, int32(16+240), rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY+240, int32(16+240), 16, rl.RayWhite)
			rl.DrawText(display, int32(50*vw-44), int32(50*vh+48), 50, rl.RayWhite)
			rl.EndDrawing()
		}
	}
}
