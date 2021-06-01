# crowdplay-bot-rs
This bot is the equivalent of my bot crowdplay-bot, but written in Rust instead
of Python.

# Why Rust?
The original codebase was pretty bloated, but that wasn't the main reason I
switched from Python to Rust. The main reason was the ability to use a
compiled language that ensured thread safety, along with greater speed. In
general, I was getting pretty annoyed from discovering errors on runtime and
certain Python modules not even annotating functions. Though I admit I never
worked with multi-threading, I figured it'd just be better to work with a
language that encourages thread safety rather than running around without my
head in Python. Plus, people seem to like Rust, so I figured I'd use this chance
to give it a try. I'd be lying if I said that wasn't the main reason though.

# Functionality
Functionality is pretty much the same aside from slightly better runtime, a lack
of a GUI (at the time of writing this). Like before, only Windows is supported
at this time, although Linux support could be implemented; sorry Mac users, I'd
do it, but the Rust libraries I need to achieve current functionality are a
little limited, plus I personally don't have a Mac to test my code with.