# golf_c
An automatic code compactor to help with golfing in c.

When doing code golfing it might often be annoying to have to remove all the whitespaces
and compact your code to one line, and also to have to have all the identifiers be single character
and have no comments. Therefore, I made a tool to help.

Currently, the tool works in a very simple way. You copy your code into your clipboard,
run the executable, and it will print the output code and put it into your clipboard so you can
paste it wherever you want.

It doesn't do any code reordering or clever things, all it does is remove comments, change the
names of all the identifiers to short names and removes whitespace.

# Building and running
Building is just done with ``cargo build``. When running you have to make sure that the
``blacklist.txt`` file is in the same folder as the executable, otherwise it will not work.
