package window

import (
	"github.com/bloodmagesoftware/leiden/internal/save"
	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/pkg/errors"
)

func Init() error {
	if err := save.LoadSettings(); err != nil {
		return errors.Wrap(err, "failed to load settings")
	}
	rl.InitWindow(save.SettingsData.Width, save.SettingsData.Height, "Leid")
	rl.SetTargetFPS(120)
	rl.SetExitKey(0)

	windowFlags := uint32(0)
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
	if save.SettingsData.AntiAliasing {
		windowFlags |= rl.FlagMsaa4xHint
	}
	rl.SetWindowState(windowFlags)

	rl.SetMasterVolume(save.SettingsData.MasterVolume)

	return nil
}

func Open() bool {
	return !rl.WindowShouldClose()
}

func Close() {
	rl.CloseWindow()
}
