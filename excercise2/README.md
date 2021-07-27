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

 TODO: Rest of show logic

&nbsp;&nbsp;&nbsp;&nbsp;If Jerry did not let his friends in, then print `The 4 of them went to the diner that night and sat in silence`.

&nbsp;&nbsp;&nbsp;&nbsp;If Jerry did let his friends in then you need to print out Jerry's comedy routine as per the results of the episode.
