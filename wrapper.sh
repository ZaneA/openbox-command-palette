#!/usr/bin/env bash
OUTPUT=$(cargo run "$0" "$1" | rofi -dmenu -p 'command palette' -mesg "<span foreground=\"#444\" size=\"small\">$1</span>" -matching regex)
#OUTPUT=$(cargo run "$0" "$1" | xmenu -r -i)
IFS='#'; arrOUTPUT=(${OUTPUT}); unset IFS
COMMAND=${arrOUTPUT[1]##*( )}
if [ -n "$COMMAND" ]; then
    sh -c "$COMMAND" &
fi
