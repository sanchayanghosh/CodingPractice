pub mod prog6 {

    fn move_to_one_side_of_array(input_vector: &mut Vec<i32>, no_of_elements: usize) {

        let mut partition_element: usize = 0;

        let mut swap: i32 = 0;

        for vector_index in 0..no_of_elements {

            if input_vector[vector_index] < 0 {
                swap = input_vector[vector_index];
                input_vector[vector_index] = input_vector[partition_element];

                input_vector[partition_element] = swap;
                partition_element += 1;
            }



        }

    }

    pub fn run_prog() {

        let mut string_of_input = String::new();

        let mut no_of_data_points: usize = 0;
        std::io::stdin().read_line(&mut string_of_input);

        no_of_data_points = string_of_input.trim().parse::<usize>().expect("Wrong number");

        let mut input_vector: Vec<i32>;

        string_of_input = String::new();

        std::io::stdin().read_line(&mut string_of_input);

        input_vector = string_of_input.trim().split(" ").into_iter().map(|X| X.trim().parse::<i32>().unwrap()).collect();


        move_to_one_side_of_array(&mut input_vector, no_of_data_points);        

        for each_int in input_vector {

            print!("{}, ", each_int);

        }


    }

}