import random
from unittest import TestCase
from algo_practice import max_min_in_array

class TestMaxMinInArray(TestCase):
    def setUp(self) -> None:
        self.max_min_in_array_obj = max_min_in_array.MaxMinInArray()

    def test_max_min_by_linear_search(self):
        empty_list = []
        list_with_one_element = [random.randint(1,1000)]
        list_with_two_elements = [random.randint(1, 20), random.randint(20, 50)]
        small_list = [round(random.random(), 2) for _ in range(random.randint(1, 20))]
        large_list = [round(random.random(), 2) for _ in range(random.randint(1000, 200000))]
        self.assertEqual(self.max_min_in_array_obj.max_min_by_linear_search(empty_list), ())
        self.assertEqual(self.max_min_in_array_obj.max_min_by_linear_search(list_with_one_element), (max(list_with_one_element), min(list_with_one_element)))
        self.assertEqual(self.max_min_in_array_obj.max_min_by_linear_search(list_with_two_elements), (max(list_with_two_elements), min(list_with_two_elements)))
        self.assertEqual(self.max_min_in_array_obj.max_min_by_linear_search(small_list), (max(small_list), min(small_list)))
        self.assertEqual(self.max_min_in_array_obj.max_min_by_linear_search(large_list), (max(large_list), min(large_list)))
