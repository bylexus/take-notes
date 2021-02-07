package main

import (
	"fmt"

	"alexi.ch/take_notes/commands"
	"alexi.ch/take_notes/types"

	"github.com/alecthomas/kong"
)

func main() {
	var cli struct {
		Debug bool `help:"Enable debug mode"`

		Create commands.CreateCmd `cmd help:"Creates a new note"`
	}

	ctx := kong.Parse(&cli)

	err := ctx.Run(&types.Context{Debug: cli.Debug})
	if err != nil {
		fmt.Printf("%v\n\n", err)
		ctx.PrintUsage(false)
		ctx.Exit(1)
	}
}
