class MaxMinInArray(object):
    def max_min_by_linear_search(self, arr):
        if not arr:
            return ()
        max, min = arr[0], arr[0]
        for each in arr:
            if each > max:
                max = each
            elif each < min:
                min = each
        return (max, min)

if __name__ == "__main__":
    pass
