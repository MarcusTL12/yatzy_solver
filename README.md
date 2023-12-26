# Yatzy rambling

Here is some rambling to collect my thought process. Mostly directed at 5 dice
yatzy (want to get that working well first), but should be extendable to 6 dice.

We will also not even try to think about what might happen if you play with the
ability to save up throws. This will blow up the state space to ridiculous
sizes.

The main thought here is to "solve" yatzy, which means for every possible state
your points is in, and the state your dice is in, you should get the
instructions of what to do next. Below I try to concretizize the various aspects
of this.

## The objective

There are many criteria you might want to use as a measure for what is the
'best' strategy. The simplest strategy is to
maximize the expected final score, that is, if you follow the stategy $n$ times
and $S_i$ is the score from game i, the value
$\bar S = \frac{1}{n} \sum_{i = 1}^n{S_i}$ is the expected value, which the
strategy aims to maximize.

However, this strategy will in a lot of cases not be the optimal strategy for
winning. Take this pathological example:

In some game you have a score of 100 and your opponent has 110 and is finished.
In your last move you have two options, one which gives you a 50/50 chance of
getting another 100 points, and the other a 100% chance of giving 11 points.
In the second case you win 100% of the time, while in the first you lose 50%
of the time, so the second choice is "obviously" the best, however the expected
final score for the first choice is 150 while for the second it is 111.

Of course the objectively best strategy is to maximize the
chances of winning against your opponents. This would in a lot of cases have to
take into account the specific states of each of your opponents, which increases
the complexity hugely, and is therefore infeasible.

The objective here is therefore to optimize a single player game.
This is, however, still not obvious what is meant by. Do you wish to optimize
the your expected value? Or perhaps minimize the probability of getting a final
score below a certain value? A cautious player might want to maximize the 95th
percentile score, meaning the score they would go over in 95% of games, while
some player striving to break records might want to maximize the 1st percentile
since they only care about getting really high scores in their best 1% of games.

We will here consider wanting to maximize the expected value of the final score
as this is a value which is particularly nice to work with, mostly since it
respects linear combinations.

## The state space
The total state space is huge, so we need to remove as much "non-interesting"
data as possible.

The behaviour of points above and below "the line" is very different, as
above the line the strategy gets much more complicated since you are trying to
get the bonus.

### Below "the line"

We'll start here since it is the easiest. Since we are optimizing for expected
scores, we do not care at all about what scores we have gotten below the line
as we can't do anything about these. We only care about which slots we can still
put points into. This means that for the state space we can treat
the cells below the line as bits in a binary number. For the 5 dice game this
is 9 cells resulting in 2^9 = 512 different combinations the "below line" part
of the state space might be in. For the 6 dice game this increases to 14 cells
giving 2^14 = 16384 combinations.

### Above "the line"

Here things start becoming more worrysome. Above the line your strategy will
depend on how many points you have, not only on which cells are spent.
For example, if you have an extra six die, a dice combination of 11222 might
be better spent on a house than trying to reroll for three ones.

This means that the total number of points above the line is another relevant
piece of the current state, in addition to which of the cells are spent.

For the 5 dice game 63 points are required to get the bonus, which means that
there are 63 different states that is needed to store: 0-62 and "enough.
Together with the six additional tiles this is then 2^6 * 64 = 4096
states above the line, giving a total of 64 * 2^(6 + 9) = 2 097 152 states.

For the 6 dice game 84 points are required, which gives a total state count of
85 * 2^(6 + 14) = 89 128 960

This does of course include some impossible states like having over 6 points
when having none of the 2-6 cells filled. I might look into how much removing
these would save, and whether it is practical to account for it.

Update: I did!

For 5 dice, the number of achievable states is 2794 / 4096 = 68.2%.
For 6 dice it is 3510 / 5440 = 64.5%.

This makes the state space sizes 2794 * 2^9 = 1 430 528 for 5 dice
and 3510 * 2^14 = 57 507 840 for 6 dice.

### Dice

In addition to the state space of possible point distributions, we also have
to consider the different ways you could roll the dice. This consists of two
parts; How the dice are thrown and how many throws you have left. Here you
will only have 0, 1 or 2 throws left.

For the die configurations, naively we have 6^5 (or 6^6 for 6 dice) combinations
= 7 776 (or 46 656). However, this massively overcounts, as the ordering of the
dice does not matter. When removing permutations this is reduced to only 252
combinations (or 462 for 6 dice), which is much more managable.

The total size of the relevant state space is then
3 * 252 * 2 097 152 = 1 585 446 912
for 5 dice and
3 * 462 * 89 128 960 = 123 532 738 560
for 6 dice.

Using the compacted statespace above "the line" this
would save around 33% space giving the state spaces
3 * 252 * 1 430 528 = 1 081 479 168
for 5 dice and
3 * 462 * 57 507 840 = 79 705 866 240
for 6 dice.

## The process

At a given state of the game you have n celles open and m throws left.
The turn moves to a new state with either m - 1 throws left
or n - 1 celles open and 2 throws left.

This means that we need to order the state space by the number of cells filled.
A subset of the state space that has a given number of cells filled and
throws left is one "layer". A layer is indicated with the tuple (m, n) where
m is 0, 1 or 2 throws left and n is the number of open cells.

The process starts with the final layer with only one state;
all cells filled, no throws left (layer (0, 0)).
Here the expected remaining score is naturally 0.

The next layer is with one free cell and no throws left (0, 1).
This is similarly trivial where there is no choice to be made,
and the expected remaining score is whatever points you get from your
set of dice in the last open cell.

The first interesting layer is with one free cell and one throw left (1, 1).
This gives you two choices; get the points from the current set of dice, or
reroll a subset of the dice, which leads you to some state of layer (0, 1).
The expected score is then the max of these two choices. The expected score of
not rerolling is the same as the layer below, just whatever amount of points
you would get. For rerolling the story becomes more convoluted as you have a
choice of which dice to reroll.

### Rerolling dice

When rerolling dice you chose between one of 2^5 (or 2^6) sets of dice to
reroll. This will both include the no-rerolling case and several identical
cases whenever there are equal dice. However this is a small amount to
loop over when checking (only 32/64 states), so the cost of reducing these
is probably not worth it. (Actually nevermind. This turned out to have a
quite significant time saving).
For a given set of dice to be rerolled there is a
well defined (and well known) probability distribution of which new set of
dice you might end up with. For example if rerolling two dice you get the
distribution of 1/36 chance of each combination of dice, with all the ones
with two different dice having a permutational symmetry giving 21 unique
rerolls. For 5-6 dice the reduction due to permutation symmetries is not a
trivial amount (7 776 / 46 656 -> 252 / 462 for rerolling 5/6 dice),
and these can easiliy be stored as small lookup arrays to be looped over.

To find the expectation value of a given set of dice to be rerolled, you loop
over all possible outcomes for the reroll (the lookup table), and find
the expected score for the new (total) set of dice (the non rerolled + rerolled)
in the layer below, then weight by the probability of that particular reroll.

To find the total expected score of a given state we loop over the possible
ways to reroll (32 or 64 combinations), then find the expected score for each
of those and pick the highest one. This will be the expected score for the
current state, and which one we picked will be the combination to reroll.

### Sizes of layers

When finding the strategy and expected scores of a given layer we require random
access to the next layer below, that is at layer (m, n) we need to be able
to access any elements of layer (m - 1, n), and at layer (0, n) we need access
to layer (2, n - 1). This means that at all times we need to be able to keep
two adjacent layers in memory. While we are just done working on layer
(m - 1, n) we start working on layer (m, n) while asynchronously writing layer
(m - 1, n) to disk.

To talk about how much memory we need to keep two layers in memory we need to
figure out what information needs to be stored in the layers. For all layers we
need to store the expected remaining score for each state, which will be stored
as a 4 byte f32. For layers (m, n), m > 0, what we need to store is which
(if any) dice to reroll. This is efficiently stored as bits in a 5/6-bit number
where each bit refers to rerolling a particular die. I am unsure whether it is
worth it to store this a packed 5/6-bit numbers or just use 8 bits per and waste
3/2 bits per byte. For 6 dice, this represents saving a total of 16 GiB across
the entire state space.

For layers (0, n) we need to store which cell to place the value into. For 5
dice there is a total of 15 cells to chose from, so that could be fit in a
4 bit number. For 6 dice there are 20 cells, so we need a 5 bit number.

This means that the most amount of memory needed is to keep layer (m, n) and
(m - 1, n) in memory for the n that has the most spaces. To get an upper
bound for this, we assume 5 bytes per layer (4 for score, 1 for strategy)
and initially no compacing the state space above the line. For this case
the largest number of states is reached when half the cells are filled
giving a size of (n choose n/2) * bonus * dice which is
(15 choose 7) * 64 * 252 = 103 783 680 states for 5 dice and
(20 choose 10) * 85 * 462 = 7 255 368 120 for 6 dice.
Keeping two of these with 5 bytes per state in memory requires
103 783 680 * 5 * 2 = 990 MiB (not bad) for 5 dice and
7 255 368 120 * 5 * 2 = 67.6 GiB for 6 dice.

Using the compacted spaces above the line, this gives
310 209 * 252 = 78 172 668 -> 746 MiB for 5 dice and
10 395 320 * 462 = 4 802 637 840 -> 44.8 GiB for 6 dice.

As a quick worst case scenario for disk bottlenecking, say we are writing
continously to disk at ~150 MiB/s, it would take 22.4 GiB / 150 MiB/s = 2.5 min
to write the previous layer to disk, giving the cpu 2.5 minutes to complete
the current layer before we are cpu bottlenecked. At time of writing this,
I assume that we will be cpu bottlenecked.

## The process 2

Dealing with a single stack of layers turned out to be a bit annoying.
What might be easier is to a matrix of layers. A layer is then indexed by
(na, nb, nt) where na and nb are the number of filled cells above/below the line
respectively and nt is the number of throws left (0, 1, 2). The layer refers to
the entire set of states fulfilling those conditions. This means within a layer
the states are indexed by (ai, bi, ti) where ai and bi are the indices for the
specific state above/below the line and ti is the index for the throw.

The max size of one of these layers is 988 * 126 * 252 -> 150 MiB (5 dice)
or 1113 * 3432 * 462 -> 8.2 GiB (6 dice).

The dependency graph for the layers would then be that to solve layer
(na, nb, nt) with nt > 0 access to layer (na, nb, nt - 1) is reqired, and to
solve layer (na, nb, 0) requires access to layers
(na - 1, nb, 2) and (na, nb - 1, 2). This puts a lower limit on the RAM
requirement where the neighbouring layers add to the largest number. This turns
out to be 440 MiB (5 dice) or 22.4 GiB (6 dice). However there is no way to
use this little RAM that does not require reading from disk. The easiest scheme
to not have to read from disk is to solve the layers "row by row" or
"column by column" whichever requires least max RAM. This turns out to be to
keep a "column" in memory at a time which would require 573 MiB (5 dice) or
33.2 GiB (6 dice).

## What about saving throws?

So what if we do want to save throws? Well, now we have to store more layers
corresponding to more throws left (nt). For the first layer this is not a
problem, as you only ever start with 2 throws left. The layers now has a more
convoluted dependency graph as in addition to rethrowing dice you can choose
to keep your remaining throws for later and put the current dice into an open
cell. That means that to solve layer (na, nb, nt) with nt > 0 needs access to
layer (na, nb, nt - 1) as before, but also (na - 1, nb, nt + 2) and
(na, nb - 1, nt + 2). This means that we have a 50% increase in memory
consumption from needing to keep 3 layers in memory at a time.

### Storage

The number of layers is massively increased. At the last turn of the game
you need to account for the possibility of having saved all the throws,
giving nt = 2 * 15 = 30 for 5 dice and nt = 2 * 20 = 40 for 6 dice.
At the middle of the game this leads to nt = 16 and nt = 20 for the largest
layers, giving a ~10x storage increase for 6 dice, which should make the
400 GiB for 6 dice become ~ 4 TiB

### The process 3

Now to systematically go through each of the different layers, the simplest is
to build up each stack of layers with the same (na, nb) for nt from 0 to the
desired max nt. Since a layer now requires random access to neighbouring layers
of nt != 0, we can not keep all neighbouring layers in memory in a simple smart
way. The simplest streaming way is to stream the next required layers and saving
the previously computed layer while working on the current layer. This means
that when computing layer (na, nb, nt) we are keeping layers (na, nb, nt - 1),
(na - 1, nb, nt + 2), (na, nb - 1, nt + 2) in memory for comptation, while
offloading layer (na, nb, nt - 1) onto disk and loading layers
(na - 1, nb, nt + 3) and (na, nb + 1, nt + 3) for the next computation.
This ammounts to keeping 6 layers in memory at a time (current, 3 using,
2 loading) where two are of size (na, nb), two of size (na - 1, nb) and two of
size (na, nb - 1). The maximum memory requirement should therefore be roughly
8.2 GiB * 6 = 49.2 GiB, which is still quite manageble.
