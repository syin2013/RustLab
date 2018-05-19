# Project 1: Bakugo's Blind Rage #

## Story ##
&nbsp;&nbsp;&nbsp; EraserHead's training for Class 1-A in the forest isn't going well.
Falco can't control his dark shadow, Uraraka can't seem to make pebbles float, and Lida can't even beat a snail in race.
Things are looking grim, and to inspire the other students EraserHead has to do something quick.
Luckily Bakugo and Midoriya seem to have a ton of energy. Maybe making them spar will inspire the others...

## Problem ##
&nbsp;&nbsp;&nbsp; Bakugo and Midoryia start to fight.
Bakugo gets an early upper hand and Midoriya runs off into the forest, but in a rare moment of brilliance Bakugo decides to go to the higher ground to get a better view of things.
He can see the entire forest, a 5 x 5 grid, and knows that Midoriya is hiding somewhere in there.
He decides to fire off explosions at random spots in the forest.
Maybe he'll hit Midoriya, maybe he won't, either way he doesn't have much energy left and can only fire off 36 explosions before Midoriya comes and Detroit Smashes his ass into next week.
But, if Bakugo hits Midoriya 3 times then he will win and be at his rightful spot in the top of the class.

&nbsp;&nbsp;&nbsp; You must simulate this duel, and print the result of who wins at the end.

## Other Rules ##
- Midoriya will stay in the same spot until he is hit. 
If he is hit he will jump to a new random spot in the forest.
- Every turn Bakugo must choose a new random spot to launch an attack on.
- Todoroki is also hiding in a random spot in the forest. If Bakugo hits him, Bakugo becomes frozen and loses three explosions. Todoroki will never move.
- Todoroki and Midoriya cannot be in the same spot.
- At the beginning of each round output a visual of the forest. '_' marks a tree 'M' marks Midoriya, 'T' marks Todoroki.
- After each round (Bakugo attack) output whether he hit, and who he it, or he missed, as well as the location of the attack.
- Also after each round output Midoriya's remaining health, and Bakugo's remaining explosions.

&nbsp;&nbsp;&nbsp; Once either Midoriya is out of health or Bakugo is out of explosions the contest is over. Print the winner followed by 'PLUS ULTRA!'

## Subpart a: ##
- Generate two random numbers and save them into variables `first_number` and `second_number` (respectively).
If the first is greater than the second print the string `first_number is greater than second_number`.

## Subpart b: ##
- Generate 3 students' (from class 1-A) coordinates. 
Assign those to variables holding their name and put those variables into a list called `student_locations`.

## Subpart c: ##
_Hint: for a coordinate `c = ('a', 'b')` you can retrieve the first value (`a`) with `c[0]` and the second with `c[1]`._ 
If you were to a have tuple that has 3 values you would retrieved that third value with `c[2]`, and so on.
- Write a function that takes Midoriya's location (a coordinate) as a parameter.
Then generate a new random coordinate and print `This is the spot!` if the newly generated spot is the same as midoriya's coordinate. 

## Subpart d: ##
- Write a new function that takes a list (namely the one from subpart b) does the same thing as subpart c for each student in the list.

## Challenge Mode: ##
- Write a function that prints a 5 by 5 grid of '_'. 
- HINT: Remember lists can contain anything! Including other lists. So what could you do with this `row = ['_', '_', '_', _', '_']`?
