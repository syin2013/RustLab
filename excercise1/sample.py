#!/bin/python3

class Summary:
    def __init__(self, user_input):
        """Write your own code
        """
        if user_input.isdigit():
            print('You entered a number! i quit')
            exit()
        self.user_input = user_input
        self.upper_reverse = user_input.upper()[::-1]
        self.num_chars = len(user_input)
        self.num_words = len(user_input.split(' '))


def main():
    summaries = []
    i = input()
    while i != 'quit':
        summaries.append(Summary(i))
        i = input()

    print('reversed upper of each string:')
    print([my_str.upper_reverse for my_str in summaries])
    print('num chars each string:')
    print([my_str.num_chars for my_str in summaries])
    print('num words each user input:')
    print([my_str.num_words for my_str in summaries])
    print('num words in all user inputs:')

    total_words_typed = sum([my_str.num_words for my_str in summaries])
    print('total_words_typed:')
    print(total_words_typed)

    if total_words_typed == 12:
        print('You typed 12 words')


if __name__ == '__main__':
    main()
