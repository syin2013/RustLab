from random import randrange


BAKUGO_MAX_BLASTS = 36
MIDORIYA_MAX_HEALTH = 3
TODOROKI_FREEZE_DECREMENT = 3
FOREST_WIDTH = 5
FOREST_DEPTH = 5


def create_coordinate():
    return (randrange(FOREST_WIDTH), randrange(FOREST_DEPTH))


def midoriya_jump(midoryia_current_location, todoroki_location):
    midoriya_new_location = todoroki_location

    while midoriya_new_location == todoroki_location or midoriya_new_location == midoryia_current_location:
        midoriya_new_location = create_coordinate()

    return midoriya_new_location


def determine_spot_label(proposed_coordinate, todoroki_location, midoriya_location):
    if midoriya_location == proposed_coordinate:
        return 'M'
    elif todoroki_location == proposed_coordinate:
        return 'T'
    else:
        return '_'


def print_forest(todoroki_location, midoriya_location):
    for y in range(FOREST_DEPTH):
        print([determine_spot_label((x, y), todoroki_location, midoriya_location)
               for x in range(FOREST_WIDTH)])


def main():
    todoroki_location = create_coordinate()
    midoriya_current_location = todoroki_location

    while midoriya_current_location == todoroki_location:
        midoriya_current_location = (randrange(FOREST_WIDTH), randrange(FOREST_DEPTH))

    print('midoriya starts at' + str(midoriya_current_location))

    midoriya_curent_health = MIDORIYA_MAX_HEALTH
    bakugo_blasts_left = BAKUGO_MAX_BLASTS

    while midoriya_curent_health > 0 and bakugo_blasts_left:
        print_forest(todoroki_location, midoriya_current_location)
        bakugo_attack_target = create_coordinate()
        print('Bakugo is attacking ' + str(bakugo_attack_target) + '!')

        if bakugo_attack_target == midoriya_current_location:
            midoriya_curent_health -= 1
            bakugo_blasts_left -= 1
            midoriya_current_location = midoriya_jump(midoriya_current_location, todoroki_location)
            print('Bakugo hit Midoriya!')
        elif bakugo_attack_target == todoroki_location:
            bakugo_blasts_left -= TODOROKI_FREEZE_DECREMENT
            bakugo_blasts_left -= 1
            print('Bakugo hit Todoroki!')
        else:
            print('Bakugo missed completely...')
            bakugo_blasts_left -= 1

        print('Midoriya health: ' + str(midoriya_curent_health))
        print('Bakugo blasts left: ' + str(bakugo_blasts_left))

    if not midoriya_curent_health:
        print('Bakugo is the winner!')
    else:
        print('Midoriya is the winner!')

    print('Plus Ultra!')


if __name__ == '__main__':
    main()
