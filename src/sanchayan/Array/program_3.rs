pub mod prog3 {

    fn find_larges_and_smallest(input_array: Vec<i8>) -> (i8, i8) {

        let mut largest = 0;
        let mut smallest = 0;

        largest = input_array[0];
        smallest = input_array[0];

        for each_input in input_array {

            if each_input > largest {
                largest = each_input;
            }

            if each_input < smallest {
                smallest = each_input;
            }

        }

        return (largest, smallest);

    }


    pub fn run_prog() {

        let mut input_data = String::new();

        std::io::stdin().read_line(&mut input_data);

        let input_size = input_data.trim().parse::<usize>().unwrap();

        input_data = String::new();

        std::io::stdin().read_line(&mut input_data);

        let mut input_array: Vec<i8> = input_data.trim().split(" ").into_iter().map(|x| x.trim().parse::<i8>().unwrap()).collect();


        let (largest, smallest) = find_larges_and_smallest(input_array);

        print!("Largest is: {} and Smallest is: {}",largest,smallest);

    }

}