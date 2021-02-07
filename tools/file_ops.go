package tools

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/kennygrant/sanitize"
	"github.com/kirsle/configdir"
)

var base_dir = configdir.LocalConfig("ch.alexi.take-notes")

func GetBaseDir() string {
	err := configdir.MakePath(base_dir)
	if err != nil {
		panic(err)
	}
	return base_dir
}

func CleanFilename(path string) string {
	return sanitize.BaseName(path)
}

func MakeNoteFilename(title string) string {
	date := time.Now().Format("20060102150405")
	cleanTitle := CleanFilename(title)
	path := filepath.Join(GetBaseDir(), fmt.Sprintf("%s-%s", date, cleanTitle))
	return path
}

func CreateNoteDir(title string) string {
	path := MakeNoteFilename(title)
	err := os.MkdirAll(path, 0755)
	if err != nil {
		panic(err)
	}

	return path
}
