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
Together with the six additional tiles this is then 2^6 * 63 = 4032
states above the line, giving a total of 63 * 2^(6 + 9) = 2 064 384 states.

For the 6 dice game 84 points are required, which gives a total state count of
84 * 2^(6 + 14) = 88 080 384

This does of course include some impossible states like having over 6 points
when having none of the 2-6 cells filled. I might look into how much removing
these would save, and whether it is practical to account for it.

Update: I did!

For 5 dice, the number of achievable states is 2794 / 4032 = 69.3%.
For 6 dice it is 3510 / 5376 = 65.3%.

This makes the state space sizes 2794 * 2^9 = 1 430 528 for 5 dice
and 3510 * 2^14 = 57 507 840 for 6 dice.

## Dice

In addition to the state space of possible point distributions, we also have
to consider the different ways you could roll the dice. This consists of two
parts; How the dice are thrown and how many throws you have left. Here you
will only have 0, 1 or 2 throws left.

For the die configurations, naively we have 6^5 (or 6^6 for 6 dice) combinations
= 7 776 (or 46 656). However, this massively overcounts, as the ordering of the
dice does not matter. When removing permutations this is reduced to only 252
combinations (or 462 for 6 dice), which is much more managable.

The total size of the relevant state space is then
3 * 252 * 2 064 384 = 1 560 674 304
for 5 dice and
3 * 462 * 88 080 384 = 122 079 412 224
for 6 dice.

This means that the total storage requirement to store the expected final score
for each state as a f32 is 5.81 GB for 5 dice and 455 GB for 6 dice.

Using the compacted statespace above "the line" this
would save around 33% space giving the state spaces
3 * 252 * 1 430 528 = 1 081 479 168
for 5 dice and
3 * 462 * 57 507 840 = 79 705 866 240
for 6 dice,
which for f32 turns into 4 GB and 297 GB.
