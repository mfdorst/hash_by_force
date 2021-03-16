#!/usr/bin/env python3

from random import randrange
from time import time


def djb2(string):
    hash = 5381
    for char in string:
        hash = hash * 33 + ord(char)
    return hash % 1000000000000000000000


def random_word():
    return ''.join([chr(randrange(33, 126)) for _ in range(32)])


def main():
    checked = 0
    total_checked = 0
    start_time = time()
    while time() - start_time < 20:
        word = random_word()
        hash = djb2(word)
        checked += 1
        total_checked += 1
        if hash < 100000000000000000:
            print(f'{word} -> {hash:021d} checked: {checked}')
            checked = 0
    end_time = time()
    time_elapsed = end_time - start_time
    checked_per_second = total_checked / time_elapsed
    print(f'{total_checked} strings checked in {time_elapsed:.2f} seconds. {checked_per_second:.0f} checked per second.')


main()
