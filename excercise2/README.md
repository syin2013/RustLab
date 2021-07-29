# Exercise 2: Jerry's Comedy Routine
![Kramer Yelling](./assets/seinfeld_yell.gif)

## Backstory:
&nbsp;&nbsp;&nbsp;&nbsp;Jerry is in a rut when it comes to his comedy act.
 The jokes are not coming the way they used to.
 He's decided to lock himself in his apartment at his desk until he's completely penned out his act for Friday night.

&nbsp;&nbsp;&nbsp;&nbsp;Meanwhile, Kramer, George, and Elaine are all crowded outside his door trying to burst in.
 Kramer has an idea for a new invention that he has to tell Jerry about, George needs to use Jerry's phone to talk to the woman he's seeing to convince her not to break up with him, and Elaine needs Jerry to convince his landlord to give her the apartment above his so she can move in at a decent price.

&nbsp;&nbsp;&nbsp;&nbsp;*Will Jerry finish his comedy act and be able to help out all of his friends?*

## Your Program:
&nbsp;&nbsp;&nbsp;&nbsp;Your program will simulate this hilarious episode of Seinfeld, and will determine how it plays out.
 However, there are a few things that we have not discussed that will be implemented in this program.
 They will be demonstrated here.

* Reading a file with a list of words into `list`
```python3
with open('text_file.txt', 'r') as f:
    individual_lines = [line.strip() for line in f.readlines()]

print(kramers_inventions)
```
* Selecting a random element from a list
```python3
import random
l = ['thing1', 'thing2', 'thing3']

random_list_element = random.choice(l)
print(random_list_element)
```

* Generating a random number
```
import random

number = random.randrange(1, 10)
print(random_number)
```

Before you start programming this save the `kramers_inventions.txt` file in

&nbsp;&nbsp;&nbsp;&nbsp;To start the program print out the sound of Kramer knocking on Jerry's door (`knock, knock, knock, knock, knock "JERRY I've got somethin for ya`), to which Jerry will respond `"Not now Kramer, Im busy"`.

&nbsp;&nbsp;&nbsp;&nbsp;Then Kramer will start to tell Jerry about his new invention idea.
 He will select anywhere from 3-7 words from the `kramers_inventions.txt` file.
 If he selects an invention with exactly 5 words as the title Jerry will say `"Thats a pretty good idea"` and let his friends in.
 If it is not 5 words Jerry will say "Come on man, Im serious here!"
 Kramer will try two more times, following the the same aforementioned logic.
 If Jerry did not let his friends in, then print `The 4 of them went to the diner that night and sat in silence` and terminate the program.

&nbsp;&nbsp;&nbsp;&nbsp;If Kramer succesfully gives a Jerry an idea he will let his friends in.
 Then Elaine will pull a copy of her lease out of her bag with a total monthly rent of $700.
 She will ask Jerry what he thinks the monthly rent will be and Jerry (who is for this very specific case, the user of the program) will input a number.
 If the number is below 600, she will print "Ill take it!", if it's between 600 and 800 she will print "ehhh... maybe", and if it is above 800 she will print "There's no way I can afford that!".

&nbsp;&nbsp;&nbsp;&nbsp;After helping Elaine, Jerry will go to help George at the phone.
 First George will need to decide if he's going to call her, which is a `50/50 chance`.
 If George calls her will lead with `"My friend Kramer has an idea i think you might like for an invention, its called the <insert the invention name here>"`

&nbsp;&nbsp;&nbsp;&nbsp;After all three characters are done print `A few days later:`

&nbsp;&nbsp;&nbsp;&nbsp;If Jerry did let his friends in then you need to print out Jerry's comedy routine as per the results of the episode.
&nbsp;&nbsp;&nbsp;&nbsp;
Jerry will summarize the days events in his comedy routine.
He will say the following
```
So my neighbor came over and pitched this dumb idea he calls the <insert invention name here>.
 Whats the deal with that?

 Then my friend Elaine [insert whether or not Elaine decided to buy the apartment].

 And George did <or did not> call the lady back.
```

Then terminate the program.


# Design requirements:
You must create at least 3 functions for this program.
 I suggest one for each character.
