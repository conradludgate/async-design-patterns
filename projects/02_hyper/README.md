# Actors between the lines

There's no changes needed here, just observe how a more idiomatic async program is actor-like.
State is owned by separate tasks and they communicate via channels (in this case, hidden from you by hyper)
