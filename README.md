TakeNotes
===========

> an easy-to-use, file-based, terminal-centric, versioned note taking system


Goals
--------

* Take notes as Markdown files, using just the text editor of your choice
* Notes can be organized hierachically by just using the file system
* Notes can be versioned, merged, pushed using git

Functionality
----------------

### Notes storage

All notes are stored in a common root directory. Notes are just directories with some files in it. Notes itself can have recursive sub-notes,
so a note directory can contain other note directories.

A directory can also be "just" a directory, without a note in it, to allow creating a hierarchical structure for your notes.
Each directory _may_ contain a `note.md` file (see below), to describe some meta data information.

The whole directory three is also a standard git repository, with all its functionalitites (like commit, push/pull to/from a remote, branching ....)

### What is a note?

A `note` is made up of the following parts:

* A directory: Each note's files are stored in a single directory. The directory name is a unique timestamp+suffix, not a human readable name
* The note itself: `dir/note.md` is just a Markdown file containing your text, and a metadata header:
  * The metadata header contains some key/value pairs, describing the note, such as a human readable title, creation date, tags ...
  * The metadata header ends with a line containing (at least) 3 dashes (`---`)
  * The rest that follows is your note text.
  * The metadata header can contain arbitary keys, which you can made up by yourself, and which you can then use for e.g. searching / sorting.
* The directory can also contain other related files, such as images or arbitary files, which can be referenced as links in your note.

An example:

```
# 1599394780-0000001/note.md:
title: A fine bunch of nonsense
created: 2020-09-06 14:20:04+02:00
tags: notes, keep, remember
---
Hi!
=====

Did you know TakeNotes? It is a **simple** note taking tool.

See an image of a kitty: ![My Kitty](./kitty.jpg)
```

### How to manage notes?

TakeNotes lets you edit your notes however you want: You can use your favourite text editor and just edit the notes files.
However, the command line utility coming with TakeNotes just **helps** you with managing your notes:

* `tn create "A fine note of its own"` will create a new note in the top level dir
* `tn create "A fine note of its own" my/folder/structure`  will create a new note in the dir `my/folder/structure/`
* `tn search "Search terms"` searches for "Seach Term" in the title and metadata fields of notes, and prints them
* `tn edit "fine note"` will search for the note give, and opens it in your configured `EDITOR`:
  1. It looks for an exact match: Folder AND name must match
  2. If not found, it searches the folder structure for an exact name match
  3. If still not found, it searches the folder structure for a partial name match
* `tn edit "my/folder/structure/fine note"` will search for the note give, and opens it in your configured `EDITOR`:
  1. It looks for an exact match: Folder AND name must match
  2. If not found, it searches the given folder for a partial match
* `tn commit "Some Commit Message (optional)"` commits the actual changes as a git commit (all changes at once, one commit)
* `tn push`: Pushes the repo to a configured remote
* `tn pull`: Pulls (fetch/merge) the repo from a configured remote
* `tn config ....`: To be defined (e.g. remotes, root folder etc.)

Architecture
--------------

TakeNotes is written in Rust.
