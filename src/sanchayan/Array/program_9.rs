pub mod prog9 {

    fn prospectively_calculate_sum()

    fn return_contigous_sum(array_data: Vec<i32>, start: usize, end: usize, memoized_cost: &mut Vec<Vec<Option<i32>>>) -> Option<i32> {

        // Here, we put arrays together with different start and end variables, and the sum of each of these arrays will be the sum of the bigger array. Out of that, we pick which is the largest of the bunch

        let mut memoized_container = memoized_cost[start];


        }

        
    }

    pub fn run_prog() {

        let mut array_size = String::new();

        std::io::stdin().read_line(&mut array_size).expect("Wrong no value");

        let mut int_array_size = 0;

        int_array_size = array_size.trim().parse::<usize>().unwrap();

        let mut array_string = String::new();

        std::io::stdin().read_line(&mut array_string).expect("Wrong value");


        let mut number_string = String::new();

        std::io::stdin().read_line(&mut number_string);
        
        let mut array_data: Vec<i32> = array_string.trim().split(" ").into_iter().map(|x| x.trim().parse::<i32>().expect("Wrong value")).collect();

        let subarray_part: Vec<Option<i32>> = vec![None; int_array_size];

        let mut memoized_sum: Vec<Vec<Option<i32>>> = vec![subarray_part; int_array_size];

        


}

