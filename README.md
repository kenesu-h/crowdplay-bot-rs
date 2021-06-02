# crowdplay-bot-rs
This is the equivalent of my crowdplay-bot project, but rewritten in Rust
instead of Python.

# Background
The bot was initially motivated by a friend of mine, who streamed themselves
playing one of my favorite games: "FTL: Faster Than Light", or "FTL" for short.
For lack of a better term, it's a spaceship commander simulator focusing on
real-time strategy and well, commanding your ship and its crew. One of the
things they always do in their playthroughs is name their crew members after
their viewers, which often made things way more personal and engaging. As a
person who enjoyed watching streams along the lines of Twitch Plays Pokemon, I
figured that it would be a cool concept for viewers to control their in-game
crew members entirely through chat messages. Unfortunately, I realized that much
of this initial idea required effectively hacking/modding the game - which I'm
not against since it's a single player game - but the modding scene is fairly
limited, and honestly hacking games is way outside of my skillset. Luckily, much
of the game could still be done purely through keyboard inputs, which pretty
much set the stage for this whole project.

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

# Differences From the Original
There are some differences from the original as a result of the rewrite. Most
were intended, but others (usually minor) resulted from adopting various Rust
libraries to get the job done, or restructuring data that could have been better
designed in retrospect.

As expected, the Rust rewrite is marginally faster than the original Python
version, although sometimes too fast for its own good. Inputs for FTL are
near-instant with some slight slowdown for depower inputs, although sometimes
these depower inputs can be dropped - probably because they're happening too
fast. Either way, I have more control over input speed as a whole.

Input handling has also been revised. I ran into some issues with the previous
handling algorithm where inputs were received in the wrong order. I did not test
the Python version enough to see if this was an issue - though I vaguely
remember that inputs were done in the right order. Either way, this has been
fixed and inputs should be queued up in a first-in first-out manner, just as it
should've been.

I also decided to drop the GUI implementation for now, as it's not needed for
the bot for function properly, and ultimately made development a bit of a
hassle. This might be picked up again in the future, but that means I have to
figure out properly starting and stopping threads for the bot and other
processes.

# Compatibility
Only Windows is supported at this time, although Linux support could be
implemented; sorry Mac users, I'd do it, but the Rust libraries I need to
achieve current functionality are a little limited, plus I personally don't have
a Mac to test my code with.

# Usage
Since there's no GUI implemented at the moment, this is a command line
application. Extract the compiled binary somewhere and run the following in your
favorite shell/terminal emulator:
```
./crowdplay-bot-rs.exe
```
On your first run, the bot should error out, but leave a `config.toml` file for
you to fill out. Edit it with a text editor and set it to your likings, but you
should be setting `token` to the OAuth token your Discord bot needs to log in. I
will not supply this. Game must also either be `FTL` or `NDS`, although it is
set to `NDS` by default.

# Compilation
Unsurprisingly, this project requires you to have Rust installed. You should be
able to compile using `cargo build`, or build and run using `cargo run`.

# Contribution
You're perfectly free to contribute or work off this code. I'm pretty sure the
license covers everything you should know about being able to work with my code,
but either way, I'd appreciate it if you credit my work if you do want to work
with it.