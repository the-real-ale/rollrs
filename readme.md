A simple dice rolling/odd calculating sci-fi ttrpg tool. Of course it will work for any ttrpg, but I themed it on Shadowrun. Now you can continue to use the command line for yet more silly things.

###Usage
####Rolling Dice
```
Usage: roll [OPTIONS] [COMMAND]

Commands:
  help-dice  Show more information on dice syntax and behavior.
  sim        Simulate and predict probabilities of possible outcomes.
  help       Print this message or the help of the given subcommand(s)

Options:
  -d, --dice <Dice>         The number and type of dice in 'x*ndm+c' format. Type help-dice for more information.
  -s, --success <Success>   Set the value of a success. This is used to report summary results and calculate predictions.
  -r, --reroll <Reroll>     Set the value to reroll at. For example, when rolling 5d6 with reroll 6, dice at 5 or 6 will be rerolled.
  -c, --count-crits <Crit>  Sets the value which counts as a critical and change variable dice behavior.
  -q, --no-shitty-crits     Change crit behavior to be in line with the popular nsc homebrew rules.
  -v, --verbose             Show individual dice rolls.
  -h, --help                Print help information
  -V, --version             Print version information
```
```
Dice Format Tutorial
Specify the number and type of dice in 'x*ndm+c' format. For example five six-sided dice is 5d6.

>> roll -v -d "5d6"
 -->
2023-01-28 21:04:33.701801549 -07:00
____________________________________
 d6                     2
 d6                     3
 d6                     5
 d6                     5
 d6                     4
Hits:           0
Total (+0):     19
____________________________________

Multiple arguments may be listed with spaces by surrounding the dice with quotations: '-d "3d4 6d6..."' Dice arguments may contain a
constant modifier by using a plus sign at the end of the dice. '2d6+4' Rolls two six-sided dice with a +4 modifier. The modifier may
be applied to multiple dice using the multiplication operator. '2*1d20+8' rolls two twenty-sided dice and applies a +8 modifier to
each roll.

>> roll -v -d "2*1d20+8" -s 20
 -->
2023-01-28 21:04:33.702045997 -07:00
____________________________________
 d20 (+8)               15
 d20 (+8)               26
Hits:           1
Total (+16):    41
____________________________________

Arguments may contain a reference to the previous number of 'successes' using the letter 'x'. The dice sequence "2*1d20+8 x*1d8+4"
rolls a d8 dice with a +4 modifier for every 'success' received on the previous set of twenty-sided dice.

>> roll -v -d "3*1d20+8 x*1d8+4" -s 14
 -->
2023-01-28 21:04:33.702137704 -07:00
____________________________________
 d20 (+8)               19
 d20 (+8)               21
 d20 (+8)               25
Hits:           3
Total (+24):    65
____________________________________
 d8 (+4)                12
 d8 (+4)                11
 d8 (+4)                9
Hits:           0
Total (+12):    32
____________________________________
```
####Showing Dice Statistics
```
Usage: roll sim [OPTIONS]

Options:
  -n, --nhits <Hits>       Set the number of hits or total value that would count as a success for the simulated roll.
  -s, --sum-total <Total>  Set the total value of success when calculating probabilities. (Cumulative dice value, not hits)
  -b, --no-bullshit        Setting this flag will silence all the fake sci-fi flair at the beginning of a report.
  -t, --show-totals        Setting this flag will show a graph of probabilities for the total values of the given dice groups.
  -p, --show-hits          Setting this flag will show a graph of probabilities for the possible hits of the given dice groups.
  -z, --hide-summary       Setting this flag will hide the probability summaries.
  -h, --help               Print help information
```
The 'simulation' shows some fake work being done to 'analyze' the data (which can be disabled using -b) and then shows the probabilities requested including some plots if specified.
```
>> roll -vd 5d6 -s 5 sim -pn 2

Establishing secure connection  ||||||||||||||||||||||||||||||||

Secret key accepted. Welcome, Random Name!

        <<<<< QEH Signal Established >>>>>
        Secure link to af74:8f66:9fa5:6367
        01/29/2075 04:08 UTC 

Confirming anonymizing techniques. If none of the following succeed, disconnect IMMEDIATELY!

Synchronized packet transmission...         Ok
Multiple hops through coNET...              Ok
Scrambled exit node address...              Ok
Correct AAA headers from tunnel traffic...  Ok
Hacked the first firewall...                Ok

Beginning TacCon aggregation sequence...
Downloading SatComm Telemetry...            ||||||||||||||||||||
Compiling local context keywords...         ||||||||||||||||||||
Uploading local radio traffic to TacCon...  ||||||||||||||||||||
Downloading convolution matrix...           ||||||||||||||||||||
Analyzing real-time TacCon data...          ||||||||||||||||||||

        <<<<< QEH Signal Invalid/Missing >>>>>
        Disconnected from af74:8f66:9fa5:6367 (Broken Pipe)
        01/29/2075 04:08 UTC 


This report was stolen for you by
         ,-.
        / \  `.  __..-,O
       :   \ --''_..-'.'
       |    . .-' `. '.
       :     .     .`.'
        \     `.  /  ..
         \      `.   ' .
          `,       `.   \
         ,|,`.        `-.\
        '.||  ``-...__..-`
         |  |
         |__|
         /||\
        //||\\
       // || \\
    __//__||__\\__
   '--------------' SSt
The North American Free Information Society


┌────────────────────────────────────────────────────────────────┐
│ Probability of success:               53.9095%                 │
│ Probability of glitch:                 3.5494%                 │
│ Probability of critical glitch:        1.6359%                 │
└────────────────────────────────────────────────────────────────┘
```

Ascii art on statistics simulation came from https://www.asciiart.eu/link-to-us in Space->Telescope category.