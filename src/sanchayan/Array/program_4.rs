pub mod prog4 {
    use std::num;


    fn parition(array_in_sort: &mut Vec<i32>, low: usize, high: usize) -> usize {

        let mut non_i = low as i8;

        non_i = non_i - 1;

        let pivot = array_in_sort[high];

        let mut temp_val;

        let mut i = 0;

        for j in low..high+1 {
            if (array_in_sort[j] < pivot) {

                non_i += 1;
                i = non_i as usize;
                temp_val = array_in_sort[i];
                array_in_sort[i] = array_in_sort[j];
                array_in_sort[j] = temp_val;

            }   
        }

        non_i += 1;

        i = non_i as usize;

        temp_val = array_in_sort[i];
        array_in_sort[i] = array_in_sort[high];
        array_in_sort[high] = temp_val;

        return i;


    }

    fn quicksort(array_in_sort: &mut Vec<i32>, low: usize, high: usize) {

        if (low < high) {

            let pivot_element = parition(array_in_sort, low, high);

            if pivot_element != 0 {

            quicksort(array_in_sort, low, pivot_element-1);

            }
            quicksort(array_in_sort, pivot_element+1, high);

        }

    }

    pub fn run_prog() {

        let mut array_size = String::new();

        std::io::stdin().read_line(&mut array_size).expect("Wrong no value");

        let mut int_array_size = 0;

        int_array_size = array_size.trim().parse::<usize>().unwrap();

        let mut array_string = String::new();

        std::io::stdin().read_line(&mut array_string).expect("Wrong value");

        let mut largest_smallest_index: usize;

        let mut number_string = String::new();

        std::io::stdin().read_line(&mut number_string);

        largest_smallest_index = number_string.trim().parse::<usize>().unwrap();
        
        let mut array_data: Vec<i32> = array_string.trim().split(" ").into_iter().map(|x| x.trim().parse::<i32>().expect("Wrong value")).collect();

        quicksort(&mut array_data, 0, int_array_size-1);

        print!("kth smallest is {} and kth largest is {}", array_data[largest_smallest_index-1], array_data[int_array_size-largest_smallest_index]);

    }


}