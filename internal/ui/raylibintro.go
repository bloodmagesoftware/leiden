package ui

import (
	"fmt"

	rl "github.com/gen2brain/raylib-go/raylib"
)

func RaylibIntro() {
	// Initialization
	title := "raylib"
	framesCounter := 0
	lettersCount := 0

	topSideRecWidth := 16
	leftSideRecHeight := 16

	bottomSideRecWidth := 16
	rightSideRecHeight := 16

	state := 0            // Tracking animation states (State Machine)
	alpha := float32(1.0) // Useful for fading

game_loop:
	for !rl.WindowShouldClose() {
		updateUnits()
		logoPositionX := int32(50*vw - 128)
		logoPositionY := int32(50*vh - 128)

		switch state {
		case 0: // State 0: Small box blinking
			framesCounter++

			if framesCounter == 120 {
				state = 1
				framesCounter = 0 // Reset counter... will be used later...
			}
		case 1: // State 1: Top and left bars growing
			topSideRecWidth += 4
			leftSideRecHeight += 4

			if topSideRecWidth == 256 {
				state = 2
			}
		case 2: // State 2: Bottom and right bars growing
			bottomSideRecWidth += 4
			rightSideRecHeight += 4

			if bottomSideRecWidth == 256 {
				state = 3
				framesCounter = 0
			}
		case 3: // State 3: Letters appearing (one by one)
			framesCounter++

			// Every 12 frames, one more letter!

			if framesCounter/12 > 0 && lettersCount < len(title) {
				lettersCount++
				framesCounter = 0
			}

			if framesCounter >= 120 && lettersCount >= len(title) { // When all letters have appeared, just fade out everything
				alpha -= 0.02

				if alpha <= 0.0 {
					alpha = 0.0
					state = 4
				}
			}
		}

		// Draw
		rl.BeginDrawing()

		rl.ClearBackground(rl.Black)

		switch state {
		case 0:
			if int32(framesCounter/15)&0x1 == 0 {
				rl.DrawRectangle(logoPositionX, logoPositionY, 16, 16, rl.RayWhite)
			}
		case 1:
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(topSideRecWidth), 16, rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY, 16, int32(leftSideRecHeight), rl.RayWhite)
		case 2:
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(topSideRecWidth), 16, rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY, 16, int32(leftSideRecHeight), rl.RayWhite)
			rl.DrawRectangle(logoPositionX+240, logoPositionY, 16, int32(rightSideRecHeight), rl.RayWhite)
			rl.DrawRectangle(logoPositionX, logoPositionY+240, int32(bottomSideRecWidth), 16, rl.RayWhite)
		case 3:
			rl.DrawRectangle(logoPositionX, logoPositionY, int32(topSideRecWidth), 16, rl.Fade(rl.RayWhite, alpha))
			rl.DrawRectangle(logoPositionX, logoPositionY+16, 16, int32(leftSideRecHeight)-32, rl.Fade(rl.RayWhite, alpha))
			rl.DrawRectangle(logoPositionX+240, logoPositionY+16, 16, int32(rightSideRecHeight)-32, rl.Fade(rl.RayWhite, alpha))
			rl.DrawRectangle(logoPositionX, logoPositionY+240, int32(bottomSideRecWidth), 16, rl.Fade(rl.RayWhite, alpha))
			rl.DrawRectangle(int32(50*vw-112), int32(50*vh-112), 224, 224, rl.Fade(rl.Black, alpha))
			rl.DrawText(fmt.Sprintf(title[:lettersCount]), int32(50*vw-44), int32(50*vh+48), 50, rl.Fade(rl.RayWhite, alpha))
		case 4:
			break game_loop
		}

		rl.EndDrawing()
	}
}
