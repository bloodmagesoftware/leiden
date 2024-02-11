package window

import (
	"errors"
	"fmt"

	"github.com/bloodmagesoftware/leiden/internal/save"
	rl "github.com/gen2brain/raylib-go/raylib"
)

func Init() error {
	if err := save.LoadSettings(); err != nil {
		return errors.Join(errors.New("failed to load settings"), err)
	}
	rl.SetTargetFPS(120)
	rl.SetExitKey(0)
	rl.InitWindow(save.SettingsData.Width, save.SettingsData.Height, "Leid")

	windowFlags := uint32(0)
	if save.SettingsData.AntiAliasing {
		windowFlags |= rl.FlagMsaa4xHint
		rl.SetConfigFlags(rl.FlagMsaa4xHint)
	}
	if save.SettingsData.VSynch {
		windowFlags |= rl.FlagVsyncHint
	}
	if save.SettingsData.Fullscreen {
		if save.SettingsData.Windowed {
			windowFlags |= rl.FlagWindowMaximized
			windowFlags |= rl.FlagBorderlessWindowedMode
			windowFlags |= rl.FlagWindowUndecorated
		} else {
			windowFlags |= rl.FlagFullscreenMode
			rl.SetWindowMonitor(0)
		}
	} else {
		if save.SettingsData.Windowed {
			windowFlags |= rl.FlagWindowResizable
		} else {
			windowFlags |= rl.FlagWindowUndecorated
		}
	}
	rl.SetWindowState(windowFlags)

	rl.SetMasterVolume(save.SettingsData.MasterVolume)

	if save.SettingsData.Width == 0 || save.SettingsData.Height == 0 {
		save.SettingsData.Width = int32(rl.GetScreenWidth())
		save.SettingsData.Height = int32(rl.GetScreenHeight())
	}
	fmt.Printf("Width: %d, Height: %d\n", save.SettingsData.Width, save.SettingsData.Height)

	return nil
}

func Open() bool {
	return !rl.WindowShouldClose()
}

func Close() {
	rl.CloseWindow()
}
