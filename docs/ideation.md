# Boggle Solver



# The Problem

As per [Wikipedia](https://en.wikipedia.org/wiki/Boggle), as linked in the email (my emphasis and ellipsis added):

> The dice settle into a 4×4 tray so that only the top letter of each cube is visible ...
>
> Each player searches for words that can be constructed from the letters of sequentially adjacent cubes, where "adjacent" cubes are those **horizontally, vertically, and diagonally neighboring**. Words must be **at least three letters long**, may include singular and plural (or other derived forms) separately, but may not use the same letter cube more than once per word. 
>
> ...
>
> One cube is printed with "Qu". This is because *[Q](https://en.wikipedia.org/wiki/Q)* is nearly always followed by *[U](https://en.wikipedia.org/wiki/U)* in English words (see [exceptions](https://en.wikipedia.org/wiki/List_of_English_words_containing_Q_not_followed_by_U)), and if there were a *Q* in *Boggle*, it would be challenging to use if a *U* did not, by chance, appear next to it. For the purposes of scoring *Qu* counts as two letters: *[squid](https://en.wikipedia.org/wiki/Squid)* would score two points (for a five-letter word) despite being formed from a chain of only four cubes. Early versions of the game had a "Q" without the accompanying "u".

### What I wasn't sure about

As I haven't played Boggle (at least at all recently), I googled to clarify that definitely adjacency includes e.g. right to left "backwards" traversal (for English). 

This was confirmed explicitly on a top Google search result: https://howdoyouplayit.com/boggle-rules-play-boggle/

It ~~is~~ was also my impression that Boggle only allows travel in a single direction for a word (a word cannot "turn") but I need[ed] to confirm or disconfirm this...  I was wrong. Page 3 of the official boggle rules as scanned on Hasbro's website: https://www.hasbro.com/common/instruct/boggle.pdf show "bowtie", "N", and "U" style spellings - so I have been thoroughly disabused of this notion.

Okay! Well this is a little "bigger" a problem than I thought, but so it goes

### Non-backtracking non-directed graph walks? which graph(s)?

Lets look at scale real quick to figure out whether we are still talking exhaustive listing vs needing heuristics in the mix

#### In which I try to figure it out by myself

So for a given board like (copied from page 3 of Boggle manual above, non-repeating conveniently )

| E    | D    | P    | G    |
| ---- | ---- | ---- | ---- |
| S    | O    | R    | A    |
| I    | L    | M    | U    |
| C    | A    | R    | T    |

we have 16 independent entry points: 12 on the edge and 4 interior. Interior nodes have 8 neighbors and edge nodes have between 3 (on corners) and 5. I think exactly 3 or 5, but I haven't exhaustively verified.

I am not great at combinatorics as a rule, but I think the worst case search path is hitting all four interior nodes in some order: so 8 * 7 * 6 * 5 *  ... it seems like there is still a factorial bound?  which 8! = 40,320  - so thinking that the max word length is 8 that would look like a 40,320 * 8 * sizeof(char) = roughly 2.5MB in most programming languages for a full enumeration of the boards valid and invalid words?

#### In which I ask the internet

That's all from me naively - and I am likely wrong. I suspect this is also googleable so lets do that. I clicked on a [couple](https://stackoverflow.com/questions/30010051/time-complexity-of-boggle-solver) things for a couple of search terms ("complexity of boggle", " how many possible words are there in a boggle"), and am looking at: http://www.robertgamble.net/2016/01/a-programmers-analysis-of-boggle.html - he clearly has 15 character paths (or 16 like ... I guess up and down the columns zig zag) though those words are rarely extant. In like .. .a sporting spirit I don't want to actually look at someone else's boggle solver source while doing this (I scrolled quickly past the one in the OP of the 1 SO I clicked) but maybe thats silly - since so far my intuition is pretty wrong.

But like I see that certainly this maps to a (fairly sub) subset of a permutation of 16 elements, so an actual intelligible upper bound is 20,922,789,888,000 "words" of 3-16 chars so like ... I'm feeling like the way to go is less enumerate the board and filter the word list than it is like build a trie of the dictionary and walk the paths the board would allow?

#### Relative sizes tho

Oh hey I realized what to actually google for my thinking and that was "how many paths through a boggle board" which gives me a google knowledge graph callout of **12 million** citing https://www-cs-faculty.stanford.edu/~zelenski/boggle/ (which I remain undecided on reading for the purposes of this exercise)

12 million is a good bit more than my low-balled 40k but its also a lot less than 20 trillion if I follow the rules right. How big is my dictionary? It says on Wikipedia that people like to go with the Scrabble dictionary (I guess cause of their mutual rejection of proper nouns?) - so how big is that? 

 ... the Wikipedia article doesn't link to a good downloadable or summary :(. It has See also "[Collins Scrabble Words](https://en.wikipedia.org/wiki/Collins_Scrabble_Words)" .. that gives me a histogram and a total of **279,496** ( - 127 two letter words = 279,369 ) (so ... much smaller)

##### An adventure to find the "right" word list 

  but boy does it not want to give me a text file. I mean so ... what do I have - 

```
❯ wc -l /usr/share/dict/words
  235886 /usr/share/dict/words

~
❯ grep -cv '^..$' /usr/share/dict/words
235726
```

I am missing 44 thousand relevant words D: 

It won't much change the algorithms or design though. It just is gonna bother me. Okay I spent another 3-5 minutes looking and came to https://scrabbleplayers.org/w/NASPA_Zyzzyva_Download which will include (but possibly hide?) the word list  not behind a login wall (like https://www.scrabbleplayers.org/cgi-bin/doc.pl?did=48 does)

*download download download grep grep grep* 

```
❯ wc -l /Volumes/NASPA\ Zyzzyva\ 3.2.4/NASPA\ Zyzzyva\ 3.2.4/Zyzzyva.app/Contents/MacOS/data/words/British/CSW19.txt
279496 /Volumes/NASPA Zyzzyva 3.2.4/NASPA Zyzzyva 3.2.4/Zyzzyva.app/Contents/MacOS/data/words/British/CSW19.txt
❯  grep -cv '^..$'  /Volumes/NASPA\ Zyzzyva\ 3.2.4/NASPA\ Zyzzyva\ 3.2.4/Zyzzyva.app/Contents/MacOS/data/words/British/CSW19.txt
279369
```

neat. (bender_camera.gif)



## The game plan (algorithm-wise)

1. Take the word list, make it into an exhaustive trie (or like ... 26 of them anyway) . This can be serialized to disk and reloaded. I am guessing its also cheap enough to compute at init - but we'll see.
2. For each of the 16 starting points in a given input, take the trie for that letter and (clone and; or soft) prune it down to the paths (if they existed in the first place) that follow from the expanding paths from the letter
3. Enumerate what is left. Bob's your uncle

This should be trivially 16x concurrent, probably could go deeper still if you wanna shard out subpaths 

### TDD?

Would be cool if I had a sample input and output to work with. I can create some manually at like 2x2 size and maybe even 3x3 but 4x4 is a big ask. sooo ... I'mma google that one ("example solved boggle"). This princeton course  https://coursera.cs.princeton.edu/algs4/assignments/boggle/specification.php is tantalizingly close to giving me one with its [board](https://coursera.cs.princeton.edu/algs4/assignments/boggle/files/board-points4540.txt)  and score. I could try to solve it myself (I see: GRAB, GRIP, SET, TEST, SERIAL) but ... I feel like that's a bad time management plan. 

I should probably start writing code now ish though. I opened this file at 4:15 [Ed: 4:07 actually] and its 5:35 now  (but I took a short break, but then I also googled the how many paths on my phone during it) - so ... yeah.

I'm calling this the 83 minute mark per [this](./timesheets/0-First 83 minutes.html) accounting (from [Timing.app](https://timingapp.com/?lang=en) ; eliding <15s entries, mostly of [Contexts.app](https://contexts.co)). Whereupon I'm calling it (possibly long?) break time (and not counting this post-script writing - cause I know I already counted non-trivial staring blankly at this document while scrolling aimlessly up and down)

