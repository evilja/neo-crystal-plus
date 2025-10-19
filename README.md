This application is written in Rustlang.

You can compile it with "cargo build" (rust compiler)

Current keybindings: you can change all keybinds to something useless using the consts in crystal_manager.rs

U or KEY_UP -> go up OR volume up (special interaction mode)

J or KEY_DOWN -> go down OR volume down (special interaction mode)

KEY_RIGHT -> +5 seconds into the song

KEY_LEFT -> -5 seconds

f -> shuffle

p -> play the selected music

s -> pause

l -> loop mode

o -> special interaction mode

b -> blacklist song (unreachable by both auto next and p button)

r -> resume

h -> search 

c -> change artist name (edit metadata)

g -> go to top

TODO

nothing

I'M AWARE

next song selection algorithm doesn't take isloop into account, which is fine if you ask me.