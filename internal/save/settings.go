package save

import (
	"encoding/json"
	"os"
	"path/filepath"

	"errors"
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
		Width:        0,
		Height:       0,
		VSynch:       true,
		Fullscreen:   true,
		Windowed:     false,
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
	home, homeSet := os.LookupEnv("HOME")
	if homeSet {
		configDir := filepath.Join(home, ".config")
		if _, err := os.Stat(configDir); err == nil {
			settingsLoc = filepath.Join(configDir, "leiden", "settings.json")
			return settingsLoc
		}
	}
	if appdata, appdataSet := os.LookupEnv("APPDATA"); appdataSet {
		settingsLoc = filepath.Join(appdata, "leiden", "settings.json")
		return settingsLoc
	}
	if homeSet {
		settingsLoc = filepath.Join(home, "leiden", "settings.json")
		return settingsLoc
	}

	settingsLoc = "settings.json"
	return settingsLoc
}

func LoadSettings() error {
	settingsPath := settingsLocation()
	if err := os.MkdirAll(filepath.Dir(settingsPath), 0755); err != nil {
		return errors.Join(errors.New("failed to retrieve settings path"), err)
	}
	_, err := os.Stat(settingsPath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return errors.Join(errors.New("failed to retrieve settings file"), err)
	}
	file, err := os.Open(settingsPath)
	if err != nil {
		return errors.Join(errors.New("failed to open settings file"), err)
	}
	defer file.Close()
	if err := json.NewDecoder(file).Decode(SettingsData); err != nil {
		return errors.Join(errors.New("failed to decode settings file"), err)
	}
	return nil
}

func SaveSettings() error {
	settingsPath := settingsLocation()
	if err := os.MkdirAll(filepath.Dir(settingsPath), 0755); err != nil {
		return errors.Join(errors.New("failed to retrieve settings path"), err)
	}
	file, err := os.Create(settingsPath)
	if err != nil {
		return errors.Join(errors.New("failed to create settings file"), err)
	}
	defer file.Close()
	enc := json.NewEncoder(file)
	enc.SetIndent("", "    ")
	if err := enc.Encode(SettingsData); err != nil {
		return errors.Join(errors.New("failed to encode settings file"), err)
	}
	return nil
}
