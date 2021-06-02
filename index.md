## Index

### Some opening words.
I was originally planning to use a Google Doc for this, but they just made embedding Gfycat videos infinitely harder. Regardless, after about four weeks of progress and indecision, I can pretty confidently say that this project is in a presentable state. This is inherently gonna be pretty lengthy, but I'm gonna try to summarize things as best as I can and not make things too boring. Knowing me though, it'll probably still end up that way.

### Background
One of my friends from college often streams herself playing one of my favorite games: "FTL: Faster Than Light", though people just refer to it as FTL. It's a bit of a complex game, so it won't be the easiest thing to describe.

#### What's FTL?
In short, FTL's a spaceship commander simulator. For those out there who are more familiar with game genres, it's a single-player roguelike where you travel across randomly-generated places in space and fight other spaceships along the way. Each fight - which involves real-time strategy - earns you scrap, which you can use to buy gear, new crew members and such at stores (also random) or upgrade parts of your ship. Point is, your goal is to get as much scrap as possible and gear up well enough so you can kill the final boss.

I feel like I'm going on way too long about FTL's objectives, so let's just dive into the gameplay. Like I mentioned, FTL's fights involve real-time strategy, though you're allowed to pause the game anytime to thing about your next move. The Gfycat below is an example of someone fighting the final boss.

<div style='position:relative; padding-bottom:calc(56.33% + 44px)'><iframe src='https://gfycat.com/ifr/ChubbyOilyAntipodesgreenparakeet' frameborder='0' scrolling='no' width='100%' height='100%' style='position:absolute;top:0;left:0;' allowfullscreen></iframe></div><p> <a href="https://gfycat.com/chubbyoilyantipodesgreenparakeet">Use this link if the Gfycat doesn't show up for you.</a></p>

Though the Gfycat doesn't illustrate this particularly well, most of FTL's strategy revolve around moving your crew members from room to room - whether for repairing or fighting off enemy crew - or managing your limited power, which you can mainly see in the bottom left. Sometimes you'll have to remove power from your oxygen system to help extinguish fires faster, or maybe you don't care about oxygen and would rather use its power for your weapons instead. This is hardly an exhaustive list, but it should at least be somewhat indicative of the various choices you have to make.

#### How FTL lead to my project.
My friend always names her crew members after her viewers, which lead to things becoming more personal between everyone; crew member deaths became a much bigger deal than usual, and as a result watching her play became much more engaging. I started thinking that it'd be cool if viewers were able to control their respective crew members through chat messages, and since I already developed Discord bots in the past, I figured it was a very feasible idea.

Unfortunately, my original idea effectively needed me to hack/mod FTL. I wasn't against hacking or modding by any means, especially since it's a single-player game. However, FTL's modding scene is currently very small and, for those who are more technically inclined, doesn't have a publicly accessible API I can use to get a direct hold of the game's information. There is an API, but it's closed source, which means I can't see or use the code behind it. This meant much of my original idea was no longer possible without an insane amount of tedium, which included allowing viewers to move their crew member around without moving anyone else's in the process - this would've required an absurd amount of hard-coding, not to mention I would have to be careful with people multiple moving their crew members around at the same time, which would easily get pretty dirty.

### Markdown

Markdown is a lightweight and easy-to-use syntax for styling your writing. It includes conventions for

```markdown
Syntax highlighted code block

# Header 1
## Header 2
### Header 3

- Bulleted
- List

1. Numbered
2. List

**Bold** and _Italic_ and `Code` text

[Link](url) and ![Image](src)
```

For more details see [GitHub Flavored Markdown](https://guides.github.com/features/mastering-markdown/).

### Jekyll Themes

Your Pages site will use the layout and styles from the Jekyll theme you have selected in your [repository settings](https://github.com/kenesu-h/crowdplay-bot-rs/settings/pages). The name of this theme is saved in the Jekyll `_config.yml` configuration file.

### Support or Contact

Having trouble with Pages? Check out our [documentation](https://docs.github.com/categories/github-pages-basics/) or [contact support](https://support.github.com/contact) and we’ll help you sort it out.
