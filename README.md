# Gadget Generator

Gadget Generator is a decompilation and native port of **Widget
Workshop: A Mad Scientist's Laboratory** (Maxis, 1995) to Rust, for
Linux, Windows, macOS and WebAssembly.

**Play it in your browser:** <https://doridian.github.io/GadgetGenerator/> (with
or without the CD image, [docs/web.md](docs/web.md)), or download it for
Linux, Windows or macOS from the
[releases](https://github.com/Doridian/GadgetGenerator/releases)
([Packages](#packages)).

The port opens straight into the editor (the original's intro and main
menu are left out): the parts tray, wiring, the worksheets, undo and the
parts' own settings dialogs; the logic, number, science, sound and time,
input and display parts; puzzle mode; widgets loaded and saved in the
original's `.WGT` format; recording and WAV import; and the help viewer
for `WIDGET.HLP`. It's a non-matching decompilation of the Win95 build's
`WIDGET.EXE`: the code behaves like the original, but doesn't compile to
the same bytes. The stack is winit, wgpu and cpal, with a resizable
window, fullscreen and UI scaling.

This repository contains **no game data files or original code**.
Without the game's CD the port runs on its own free replacements:
OpenWGFX (the pictures and pointers), OpenWSFX (the sounds) and
OpenWDATA (the texts, dialogs, help, puzzles and sample widgets), built
into the program (`crates/engine/opengfx`, `opensfx`, `opendata`; how
they're made: `tools/opengfx`, `tools/opensfx`, `tools/opendata`). With
your own copy of the CD it uses the original's.

## Getting the game

The port reads the **Mac/PC CD** (volume `WIDGETS`), as an `.iso` or as
a folder with its files. The Internet
Archive has the `.iso`:
[widget-workshop](https://archive.org/details/widget-workshop).

It also reads the much smaller **OS/2 Warp CD** (`WW1_01A.ISO`, game folder
`WIDGET/`): its modules are OS/2 executables whose dialogs, menu, strings
and pointers the port turns into the Win95 build's
([docs/os2.md](docs/os2.md)).

The OS/2 CD has no Windows fonts, so the port bundles free stand-ins
(`crates/engine/fonts`): Wine's MS Sans Serif and Small Fonts, and Widget
Sans, rendered from Liberation Sans, for the gadgets' Arial. When the
Mac/PC CD's own fonts (`WIN31/SVGA`) are there, they win.

## Packages

Every push builds the port as one file per platform
(`.github/workflows/build.yml`); a `v*` tag publishes them on its release:

- `gadget-generator-x86_64.AppImage`, `gadget-generator-aarch64.AppImage`
  (built on Ubuntu 22.04: glibc 2.35 or newer)
- `gadget-generator-windows-x86_64.exe` (the C runtime linked in)
- `gadget-generator-macos.dmg`: `Gadget Generator.app`, for Apple silicon and
  Intel (signed ad hoc only: the first start needs a right click, Open)
- `gadget-generator-web.zip`: the browser build, as on the Pages site

None of them carries game data. Started without a CD (from its icon) and
with no `.iso` or `orig` in the working directory, the port runs on its
own packs; Settings ▸ Change CD... asks for the CD image with the
system's file dialog and keeps its path in its data directory (`cd-path.txt` in `~/.local/share/gadget-generator`,
`%APPDATA%\Gadget Generator`, `~/Library/Application Support/Gadget Generator`;
the directories of the port's old name, `widget-workshop` or
`Widget Workshop`, are moved there on the first start);
it starts on that CD while the file is there. On Linux the dialog is the
desktop's (xdg-desktop-portal, else zenity); with neither (a minimal
system), Change CD... asks for the CD image to be dropped on the game's
window. The icon is
the port's own (`tools/make-icon.py`).

## Running

```sh
cargo run --release -p widget                       # the .iso here, else orig/, else no CD
cargo run --release -p widget -- CD.iso MY.WGT      # a CD and a widget to open
cargo run --release -p widget -- CD.iso 'https://…/widget.html#w=…'   # a widget link
cargo run --release -p widget -- --help
```

Options: `--open`, `--opengfx`, `--opensfx` and `--opendata`
([below](#without-the-cd-openwgfx-openwsfx-openwdata)), `--lang=en|sv|fr`,
`--no-audio`, `--window-size=WxH`, `--fullscreen`, `--windowed`,
`--ui-scale=N|auto` (remembered in the settings file), `--overrides=DIR`
(resources that replace or add to the CD's) and `--export=DIR` (writes
every resource there in that layout and quits). On Linux the sound needs
ALSA's headers to build (`libasound2-dev`).

Options ▸ Settings, the port's own, changes the UI scale on the spot
(Automatic, or 1× to 4× in quarters), the language without a CD (with
one, the CD's decides; the widget on the worksheet stays, a puzzle is
loaded again in the new language), and switches the port's
enhancements of the original on or off, all of them at once. They're
on until the player turns them off (`original=true` in the settings
file). The enhancements: the parts tray shows a whole folder at once, its pages one
under the other, without the page arrows; and the toolbars, dialogs and
message boxes are dragged whole, not as an outline.
**Full screen** switches the window to full screen and back, as F11 and
Alt+Enter do (on a phone, where there are no such keys).
Its **Change CD...** button, after "Save changes?", picks another CD
image and starts the game again on it (in the browser, the page's start
form comes back); **Play without the CD** starts it again on the port's
own packs, and it stays without a CD, on later starts too, until Change
CD... picks one (the browser keeps the stored image meanwhile). The
packs then come in the language picked before, else the system's.

On a touch screen a tap is a click, a long press a double click, and a
tap on a text field opens the device's own text box (in the browser).

Widgets travel as links too ([docs/web.md](docs/web.md#widget-links)):
File ▸ Copy Link to Widget puts one on the clipboard, which opens the
widget in the browser build for anyone with the CD, and File ▸ Open
Widget Link (Ctrl+V) opens the one on the clipboard, natively too.

The browser build: `tools/build-web.sh`, [docs/web.md](docs/web.md).

## Without the CD: OpenWGFX, OpenWSFX, OpenWDATA

The port has its own pictures (OpenWGFX), sounds (OpenWSFX) and data
(OpenWDATA: the dialogs, menus, messages, the parts' texts and tables,
the help, puzzles and sample widgets), built into the program, as OpenTTD
has OpenGFX, OpenSFX and OpenMSX. Started with no CD given or found (or
with `--open`) it runs on them alone; with a CD, `--opengfx`, `--opensfx`
and `--opendata` use them instead of the CD's parts one at a time (in the
browser `?open`, `?opengfx`, ...). Every part, toolbar, pointer, dialog
and sound keeps the original's size, number and job (the knots where the
wires plug in, the places the digits are drawn, the dialogs' controls),
so widgets made with the original open and work the same; the pictures
are drawn, the sounds synthesized and the texts, help, puzzles and
samples written anew. The game has no music besides its intro, which the
port leaves out, so there is no music set.

```sh
cargo run --release -p widget                         # no CD here: the packs alone
cargo run --release -p widget -- --open               # the packs even with a CD here
cargo run --release -p widget -- --opengfx --opensfx CD.iso   # the CD's data, own pictures and sounds
cargo run --release -p widget -- --opengfx=DIR CD.iso # a pack from a folder
```

The packs are in `crates/engine/opengfx`, `crates/engine/opensfx` and
`crates/engine/opendata`; how they're made, their layout and rules:
[tools/opengfx/README.md](tools/opengfx/README.md),
[tools/opensfx/README.md](tools/opensfx/README.md),
[tools/opendata/README.md](tools/opendata/README.md).

## Languages

The port speaks English, Swedish and French: as far as is known, the
languages Widget Workshop itself came out in, as Widget Workshop: A Mad
Scientist's Laboratory, Uppfinnarverkstan and Eurêka, le Laboratoire du
savant fou. (If you know of another edition, please say so in an
issue.) With a CD it plays in the
CD's language (the Swedish edition, Uppfinnarverkstan, and the French
one, Eurêka, le Laboratoire du savant fou, as well: the Swedish puzzle
file is read too); the port's own words, what the original took from
Windows and what the port adds, follow it (`engine::lang`). Without a CD
the packs come in the language chosen: `--lang=sv` or `--lang=fr`
(`?lang=sv`, `?lang=fr` in the browser, whose start page has a list for
it), else the system's. Swedish OpenWDATA is `tools/opendata/sv`
(`crates/engine/opendata-sv`), Swedish OpenWGFX the pictures with words,
`crates/engine/opengfx-sv`, over the English ones; French OpenWDATA is
`tools/opendata/fr` (`crates/engine/opendata-fr`) and French OpenWGFX
`crates/engine/opengfx-fr`.

## Tools

- `tools/gh`, `tools/ghidra/`: headless Ghidra setup and CLI for the decompilation.
- `tools/build-web.sh`: builds the browser version into `web/`.
- `tools/make-icon.py`: draws the port's icon.

## Docs

- [docs/web.md](docs/web.md): the browser build
- [docs/gadget-model.md](docs/gadget-model.md): the gadget model
- [docs/editor-layout.md](docs/editor-layout.md): the editor: main window, toolbars, parts tray, worksheet frame
- [docs/editor-interaction.md](docs/editor-interaction.md): editor interaction: the worksheet
- [docs/dialogs.md](docs/dialogs.md): the parts' settings dialogs
- [docs/puzzles.md](docs/puzzles.md): puzzles
- [docs/help.md](docs/help.md): `WIDGET.HLP` and the Help menu
- [docs/os2.md](docs/os2.md): the OS/2 Warp CD and how its modules differ
- [docs/gadgets/logic.md](docs/gadgets/logic.md): the logic and number parts
- [docs/gadgets/science.md](docs/gadgets/science.md): the science lab parts
- [docs/gadgets/sound-time.md](docs/gadgets/sound-time.md): time and sound parts
- [docs/gadgets/io.md](docs/gadgets/io.md): input and display parts

## Legal

The port's code is under the AGPL-3.0-or-later ([LICENSE](LICENSE)).
Widget Workshop is © Maxis; Gadget Generator is an unofficial port, and no
game data is included.
