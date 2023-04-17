import argparse


class ReverseArray(object):
    def reverse_by_slicing(self, arr):
        """
        As described in https://stackoverflow.com/questions/509211/understanding-slicing
        Complexity: O(n)
        Space Complexity: O(n)
        """
        return arr[::-1]

    def reverse_by_iteration(self, arr):
        """
        Complexity: O(n)
        Space Complexity: O(n)
        """
        reversed_array = []
        for i in range(len(arr)-1, -1, -1):
            reversed_array.append(arr[i])
        return reversed_array

    def reverse_in_place(self, arr):
        """
        Complexity: O(n)
        Space Complexity: O(1)
        """
        start, end = 0, len(arr) - 1
        while start <= end:
            arr[start], arr[end] = arr[end], arr[start] # This is python's builtin way of managing swap without using a 3rd variable
            start += 1
            end -= 1
        return arr

    def _reveser_array(self, arr, start, end):
        if start >= end:
            return
        arr[start], arr[end] = arr[end], arr[start] # This is python's builtin way of managing swap without using a 3rd variable
        self._reveser_array(arr, start+1, end-1)

    def reverse_with_recursion(self, arr):
        self._reveser_array(arr, 0, len(arr) - 1)
        return arr


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('array', default=[], type=str)
    arr = parser.parse_args().array.split(',')
    reverser = ReverseArray()
    reverser.reverse_with_recursion(arr)
    reverser.reverse_in_place(arr)
    reverser.reverse_by_iteration(arr)
    reverser.reverse_by_slicing(arr)
