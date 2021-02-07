package types

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"alexi.ch/take_notes/tools"
)

type Note struct {
	Dir   string
	Title string
}

func NewNote(title string) (*Note, error) {
	note := new(Note)
	note.Title = title
	note.Dir = tools.CreateNoteDir(title)
	return note, note.createNotesFile()
}

func (n *Note) createNotesFile() error {
	notesFile := filepath.Join(n.Dir, "note.md")
	if _, err := os.Stat(notesFile); err == nil {
		return nil
	} else if os.IsNotExist(err) {
		f, err := os.Create(notesFile)
		if err == nil {
			// now finally, create the content
			defer f.Close()
			return n.initNoteFile(f)
		}
		return err
	} else {
		return err
	}
}

func (n *Note) initNoteFile(f *os.File) error {
	now := time.Now().Format(time.UnixDate)
	_, err := f.WriteString(fmt.Sprintf(
		"title: %s\n"+
			"created: %s\n"+
			"tags: \n"+
			"---\n\n"+
			"# %s\n\n",
		n.Title,
		now,
		n.Title,
	))
	return err
}
