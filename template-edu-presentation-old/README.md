# Talk Template

Here is a template I use for giving talks.  The work flow follows:

1) Generate an essay for the talk you'll give in essay.md (please write it in the exact manor in which you intend to read it).
2) Generate slides as you're writing the essay.md, they're located in the `slides/index.html`
3) Profit .....just kidding =(


## essay.md

The top part of essay.md should be a bulleted outline of the talk.  Then there should be links to slides or other resources such as references.  The remainder of `essay.md` should be the actual talk written as it would be spoken during the lecture.  Once essay.md is written it's built into the index.html via the `build/make.sh` script.

```
cd build/
./make.sh
```


## slides/

The slides are built using reveal.js.  Hack your slides into the `slides/index.html` file.


## brief.md

The brief.md gives a brief description of the talk.  This is handy for when you need to write summaries and stuff so that your talk can be printed onto pamphlets and things for conventions and workshop apps.


## Publish

Once everything is all together, you should be able to push this to your github repo and reach the essay via, eg... https://thenotary.github.io/name-of-talk


## Screen Recording

`recordmydesktop --on-the-fly-encoding --fps 1` might work for recording the screen and audio while minimizing the file size.  NB needs better recording equipemnt...

