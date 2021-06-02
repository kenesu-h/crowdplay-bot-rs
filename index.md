# Index
This page is meant for people who follow my second IG account. This might not
be the most professional thing as a result, and definitely not the most succinct.

## Opening Words
I was originally planning to use a Google Doc for this, but they just made
embedding Gfycat videos infinitely harder. Regardless, after about four weeks of
progress and indecision, I can pretty confidently say that this project is in a
presentable state. This is inherently gonna be pretty lengthy, but I'm gonna try
to summarize things as best as I can and not make things too boring. Knowing me
though, it'll probably still end up that way.

## Background
One of my friends from college often streams herself playing one of my favorite
games: "FTL: Faster Than Light", though people just refer to it as FTL. It's a
bit of a complex game, so it won't be the easiest thing to describe.

### What's FTL?
In short, FTL's a spaceship commander simulator. For those out there who are
more familiar with game genres, it's a single-player roguelike where you travel
across randomly-generated places in space and fight other spaceships along the
way. Each fight - which involves real-time strategy - earns you scrap, which you
can use to buy gear, new crew members and such at stores (also random) or
upgrade parts of your ship. Point is, your goal is to get as much scrap as
possible and gear up well enough so you can kill the final boss.

I feel like I'm going on way too long about FTL's objectives, so let's just dive
into the gameplay. Like I mentioned, FTL's fights involve real-time strategy,
though you're allowed to pause the game anytime to thing about your next move.
The Gfycat below is an example of someone fighting the final boss.

<div style='position:relative; padding-bottom:calc(56.33% + 44px)'><iframe src='https://gfycat.com/ifr/ChubbyOilyAntipodesgreenparakeet' frameborder='0' scrolling='no' width='100%' height='100%' style='position:absolute;top:0;left:0;' allowfullscreen></iframe></div><p> <a href="https://gfycat.com/chubbyoilyantipodesgreenparakeet">Use this link if the Gfycat doesn't show up for you.</a></p>

Though the Gfycat doesn't illustrate this particularly well, most of FTL's
strategy revolve around moving your crew members from room to room - whether for
repairing or fighting off enemy crew - or managing your limited power, which you
can mainly see in the bottom left. Sometimes you'll have to remove power from
your oxygen system to help extinguish fires faster, or maybe you don't care
about oxygen and would rather use its power for your weapons instead. This is
hardly an exhaustive list, but it should at least be somewhat indicative of the
various choices you have to make.

### How FTL lead to my project.
My friend always names her crew members after her viewers, which lead to things
becoming more personal between everyone; crew member deaths became a much bigger
deal than usual, and as a result watching her play became much more engaging. I
started thinking that it'd be cool if viewers were able to control their
respective crew members through chat messages, and since I already developed
Discord bots in the past, I figured it was a very feasible idea.

Unfortunately, my original idea effectively needed me to hack/mod FTL. I wasn't
against hacking or modding by any means, especially since it's a single-player
game. However, FTL's modding scene is currently very small and, for those who
are more technically inclined, doesn't have a publicly accessible API I can use
to get a direct hold of the game's information. There is an API, but it's closed
source, which means I can't see or use the code behind it. This meant much of my
original idea was no longer possible without an insane amount of tedium, which
included allowing viewers to move their crew member around without moving anyone
else's in the process - this would've required an absurd amount of hard-coding,
not to mention I would have to be careful with people multiple moving their crew
members around at the same time. Not only would this require mouse/touchpad
inputs, which was out of the question since we can only use chat messages, but
this would also get really dirty since I imagine the game wouldn't like multiple
people doing so many mouse-related tasks at once.

However, much of the game was playable with the keyboard alone, which meant
viewers could still make an impact on gameplay through chat, so I went ahead
and, well, developed it as a Discord bot.

## Development
I wrote the bot in Python, and things went pretty well... kinda. There was an
annoying amount of logic that made up the commands, but the important part is
that I ultimately got the bot to work with FTL.

### How It's Used
Here's a primer on the bot is meant to be used:
1. Each command to the bot must start with a prefix (I set it to ";").
2. Immediately after the prefix, you have to specify which ship system to want
   to target - i.e: "engines", so the command so far would be ";engines".
3. After the system name, you have to specify how many bars of power you want to
   add or remove - i.e: ";engines 2" would add 2 bars of power to engines (if
   possible), but ";engines -3" would remove 3 bars of power.

That covers most commands, but there are some exceptions. I won't go over all of
them since this is already pretty long and I'm pretty sure you get the point.

Some systems can be activated. Cloaking will activate a cloaking effect on
your ship (obviously), which gives you a big chance (60%) to dodge incoming
hits. However, cloaking can also take in or lose power.

To activate cloaking, I would just remove the number after the system name, so
that'd result in ";cloaking". To power/depower it, I would give the number, so
";cloaking 3" would add 4 bars of power.

This is especially important since activatable systems tend to have a major
effect on gameplay.

### Supporting Pokemon and DS Games
After I finished the FTL portion of the bot, I figured that the whole concept
was applicable to pretty much any other game. Plus, if we're honest here, FTL is
not a very well-known game, so I chose to support Pokemon next, which everyone
probably recognizes. Basically, I made the bot the equivalent of
[Twitch Plays Pokemon](https://en.wikipedia.org/wiki/Twitch_Plays_Pok%C3%A9mon),
which is a well-known bot that practically accomplishes the same thing as mine,
but was initially intended for Pokemon and done as a bot for Twitch.tv instead.

The Pokemon portion was MUCH simpler by comparison, so I ended up spending way
less time implementing it. Keep in mind, Pokemon was run using a Nintendo DS
emulator, which is (for those who don't know) basically a program that lets your
computer play DS games. You can probably use this for any Nintendo DS game as
long as you're using a DS emulator (namely DeSmuME) and you aren't playing
something that relies on the touch screen. 

For a quick primer on this:
1. Include the prefix as always.
2. Specify the name/letter of the button you want to press - i.e: ";a" will
   press the A button once.
3. If you want to press the button more than once, you can optionally give a
   number after the command - i.e: ";right 10" will press right on the D-pad 10
   times.

### Continued Development
Supporting DS games necessitated being able to switch between "FTL" and "DS"
mode, so I made ended up implementing settings and eventually a GUI. In layman's
terms, a GUI is effectively a window for a program, plus all the buttons and
such that make it up. It was at this point though that I started running into a
ton of problems, probably out of overambition.

### Problems
This is probably the part where it stops being boring and more fun for you to
read.

1. **GUI code is absolutely fucking garbage.** \
   I worked with GUI code before, but I never imagined it would be so tedious to
   write such a simple, awful looking window.
   ![GUI Code](/images/gui-code.png) \
   It goes on longer than this, by the way (almost 100 more lines). \
   ![The resulting GUI.](/images/gui.png) \
   All of that for this???
2. **The project itself is bloated as hell.** \
   I know I like to preach object-oriented design to my friends who deal with
   programming, but if you're going to design a project with OOD, keep your
   classes simple, otherwise you might end up with this:
   ![Original project size.](/images/python-size.png) \
   This isn't even all of it LOL
3. **Python was not as neat as I would've liked it to be.** \
   This is more of a technical topic, but I just found myself getting frustrated
   with Python for a number of reasons. I had to use other people's
   modules/libraries for various things, such as getting button presses to work
   or setting up the Discord bot-side of things. However, some of these
   libraries weren't well-documented, so I would have no idea what data their
   functions took in and returned. I felt like by this point, my codebase was
   getting way too messy (partially because of me) and the fact that Python only
   finds certain errors when you run the code... it all summed up to be a pretty
   painful experience as opposed to an initially fun one.
4. **I also found that key presses were coming out too slow for my liking.** \
   This is pretty minor compared to the other reasons, and I probably shouldn't
   have considered it as that much of a problem, but key presses sometimes came
   out pretty slow. I have a feeling this is because Python is a higher-level
   (in layman's terms, not fast) language.

For these reasons, I just rewrote the whole bot in a trending programming
language called Rust.

Yes, I picked up a whole programming language just to rewrite the bot. I did
however want to learn it anyway since it's pretty popular nowadays and enforces
some important CS concepts that aren't really relevant for you guys to know. It
also happens to be a lower-level language, so it's supposed to be faster.

## Rewriting
This is already going on INSANELY long, so I'll try to be short here.

The rewrite did end up being MUCH faster, and while I lost a ton of my
brain cells trying to learn Rust for this project (which was a bit of a
mistake), doing so cut down my project size a ton and I have to admit, there's
plenty parts of Rust that I like as opposed to Python. \
![The Rust rewrite's size.](/images/rust-size.png) \
Seriously, this is much smaller than the Python version.

I also ended up fixing bugs as a result of the rewrite, and in general this
version seems to be a lot cleaner. I did have to dump the whole GUI thing, but
to be honest, that was a lot more of a pain in the ass than it was worth.

## Closing Words
That pretty much sums up the whole project. While I'm thinking of gradually
adding more features (and thus, supporting more games) to the bot, I can't
really promise anything, and for all I know I'll probably be moving onto another
project, or spending time to focus on more pressing issues in my life at the
moment. That said, this bot is in a very usable state, albeit only for Windows
users. Sorry Mac users, I'm afraid I don't have a Mac to test this out with, so
this will NOT work for you guys (unless you're open to working together on
this).

You can download the bot on Windows and find out more about the some, but not
all, of the finer details on the project's GitHub page - you can click the 
button at the top of the page to go there. If you have any suggestions or you're
a someone who wants to know more (maybe contribute) to this project, feel free
to contact me directly and/or leave a comment on the IG post.

For now though, that's it. I already have something in mind as a continuation of
this project's functionalities, but no promises I'll even get to it. Anyways,
take care y'all.