# openbox-command-palette

Parses Openbox menu config, or pipe menus, flattening output into a
list suitable for use by other scripts.

If you're like me and use Openbox menus (and pipe menus) extensively
on your desktop this makes it easier to not waste that effort while
using other desktop environments :)

Right now there isn't anything to configure, e.g. separators. Please
open an issue if you'd find this useful.

# Installing

Using `cargo` (a `shell.nix` is provided for Nix users):

```sh
# will install to ~/.cargo/bin/openbox-command-palette
cargo install --git https://github.com/ZaneA/openbox-command-palette
```

# Usage

## Basic usage

```sh
# Usage: openbox-command-palette <path to wrapper> <path to openbox menu.xml or pipe menu command>
```

For example:

```sh
# openbox-command-palette wrapper.sh ~/.config/openbox/menu.xml
terminal                                     # xterm
files                                        # thunar
a pipemenu                                   # wrapper.sh "pipemenu-script"
system audio volume                          # pavucontrol
system audio patchbay                        # helvum
```

## Usage with a wrapper

`openbox-command-palette` expects to be used within a user-defined
wrapper script that will pass the output to a suitable UI. This is
crucial for navigating through pipe menus.

An example using rofi/dmenu or xmenu would look like:

```sh
#!/usr/bin/env bash
# rofi
OUTPUT=$(~/.cargo/bin/openbox-command-palette "$0" "$1" | rofi -dmenu)
# xmenu
#OUTPUT=$(~/.cargo/bin/openbox-command-palette "$0" "$1" | xmenu -r -i)
IFS='#'; arrOUTPUT=(${OUTPUT}); unset IFS
COMMAND=${arrOUTPUT[1]##*( )}
if [ -n "$COMMAND" ]; then
    sh -c "$COMMAND" &
fi
```

This will provide a filterable command-palette style launcher, and
allow you to navigate through pipe menus (i.e. re-launching the
wrapper with the new menu).
