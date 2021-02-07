package commands

import (
	"errors"
	"fmt"
	"strings"

	"alexi.ch/take_notes/types"
)

type CreateCmd struct {
	Title []string `arg name:"title" required help:"The title of the new note"`
}

func (c *CreateCmd) Run(ctx *types.Context) error {
	if len(c.Title) == 0 {
		return errors.New("Please provide a title")
	}

	title := strings.Join(c.Title, " ")

	note, err := types.NewNote(title)
	if err != nil {
		panic(err)
	}

	fmt.Printf("New note created: %s\n", note.Dir)
	return nil
}
