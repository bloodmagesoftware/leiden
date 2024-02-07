package save

import (
	"encoding/json"
	"os"
	"path/filepath"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/pkg/errors"
)

var (
	SettingsData *Settings = defaultSettings()
	settingsLoc  string
)

type Settings struct {
	Width        int32   `json:"width"`
	Height       int32   `json:"height"`
	VSynch       bool    `json:"vsynch"`
	Fullscreen   bool    `json:"fullscreen"`
	Windowed     bool    `json:"windowed"`
	AntiAliasing bool    `json:"antialiasing"`
	MasterVolume float32 `json:"mastervolume"`
	MusicVolume  float32 `json:"musicvolume"`
	SFXVolume    float32 `json:"sfxvolume"`
	DialogVolume float32 `json:"dialogvolume"`
}

func defaultSettings() *Settings {
	return &Settings{
		Width:        720,
		Height:       480,
		VSynch:       true,
		Fullscreen:   false,
		Windowed:     true,
		AntiAliasing: true,
		MasterVolume: 1.0,
		MusicVolume:  1.0,
		SFXVolume:    1.0,
		DialogVolume: 1.0,
	}
}

func settingsLocation() string {
	if settingsLoc != "" {
		return settingsLoc
	}
	if xdgConfigHome, ok := os.LookupEnv("XDG_CONFIG_HOME"); ok {
		settingsLoc = filepath.Join(xdgConfigHome, "leiden", "settings.json")
		return settingsLoc
	}
	settingsLoc = filepath.Join(rl.HomeDir(), "leiden", "settings.json")
	return settingsLoc
}

func LoadSettings() error {
	settingsPath := settingsLocation()
	if err := os.MkdirAll(filepath.Dir(settingsPath), 0755); err != nil {
		return errors.Wrap(err, "failed to retrieve settings path")
	}
	_, err := os.Stat(settingsPath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return errors.Wrap(err, "failed to retrieve settings file")
	}
	file, err := os.Open(settingsPath)
	if err != nil {
		return errors.Wrap(err, "failed to open settings file")
	}
	defer file.Close()
	if err := json.NewDecoder(file).Decode(SettingsData); err != nil {
		return errors.Wrap(err, "failed to decode settings file")
	}
	return nil
}

func SaveSettings() error {
	settingsPath := settingsLocation()
	if err := os.MkdirAll(filepath.Dir(settingsPath), 0755); err != nil {
		return errors.Wrap(err, "failed to retrieve settings path")
	}
	file, err := os.Create(settingsPath)
	if err != nil {
		return errors.Wrap(err, "failed to create settings file")
	}
	defer file.Close()
	if err := json.NewEncoder(file).Encode(SettingsData); err != nil {
		return errors.Wrap(err, "failed to encode settings file")
	}
	return nil
}
