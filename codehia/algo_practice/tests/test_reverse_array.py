from unittest import TestCase
import sys
import random
from algo_practice import reverse_array

from random import choice
from string import ascii_lowercase, digits

CHARS = ascii_lowercase + digits

class TestArrayReverse(TestCase):
    def setUp(self) -> None:
        self.reverser = reverse_array.ReverseArray()

    def test_reverse_by_iteration(self):
        string_array = [''.join(choice(CHARS) for _ in range(2)) for _ in range(100)]
        small_array = [random.random() for _ in range(random.randint(1, 20))] 
        large_array = [random.random() for _ in range(random.randint(1000, 200000))] 
        empty_array = []
        self.assertEqual(self.reverser.reverse_by_iteration(small_array), small_array[::-1])
        self.assertEqual(self.reverser.reverse_by_iteration(large_array), large_array[::-1])
        self.assertEqual(self.reverser.reverse_by_iteration(empty_array), empty_array[::-1])
        self.assertEqual(self.reverser.reverse_by_iteration(string_array), string_array[::-1])

    def test_reverse_by_slicing(self):
        string_array = [''.join(choice(CHARS) for _ in range(2)) for _ in range(100)]
        small_array = [random.random() for _ in range(random.randint(1, 20))] 
        large_array = [random.random() for _ in range(random.randint(1000, 200000))] 
        empty_array = []
        self.assertEqual(self.reverser.reverse_by_slicing(small_array), small_array[::-1])
        self.assertEqual(self.reverser.reverse_by_slicing(large_array), large_array[::-1])
        self.assertEqual(self.reverser.reverse_by_slicing(empty_array), empty_array[::-1])
        self.assertEqual(self.reverser.reverse_by_slicing(string_array), string_array[::-1])

    def test_reverse_in_place(self):
        string_array = [''.join(choice(CHARS) for _ in range(2)) for _ in range(100)]
        small_array = [random.random() for _ in range(random.randint(1, 20))] 
        large_array = [random.random() for _ in range(random.randint(1000, 200000))] 
        empty_array = []
        self.assertEqual(self.reverser.reverse_in_place(small_array.copy()), small_array[::-1])
        self.assertEqual(self.reverser.reverse_in_place(large_array.copy()), large_array[::-1])
        self.assertEqual(self.reverser.reverse_in_place(empty_array.copy()), empty_array[::-1])
        self.assertEqual(self.reverser.reverse_in_place(string_array.copy()), string_array[::-1])


    def test_reverse_with_recursion(self):
        string_array = [''.join(choice(CHARS) for _ in range(2)) for _ in range(100)]
        small_array = [random.random() for _ in range(random.randint(1, 20))] 
        large_array = [random.random() for _ in range(random.randint(1000, sys.getrecursionlimit()))] 
        empty_array = []
        self.assertEqual(self.reverser.reverse_with_recursion(small_array.copy()), small_array[::-1])
        self.assertEqual(self.reverser.reverse_with_recursion(large_array.copy()), large_array[::-1])
        self.assertEqual(self.reverser.reverse_with_recursion(empty_array.copy()), empty_array[::-1])
        self.assertEqual(self.reverser.reverse_with_recursion(string_array.copy()), string_array[::-1])
