pub mod prog5 {


    fn sort012(input_vector: &mut Vec<i8>, bucket_array: &mut [i8; 3]) {

        let mut usize_index = 0;

        for each_input in input_vector.as_slice() {

            usize_index = *each_input as usize;

            bucket_array[usize_index] += 1;

        }

        let mut initial_index = 0;

        

        for each_index in 0..3 {

            usize_index = each_index;

            for each_no in 0..bucket_array[usize_index] {

                input_vector[initial_index as usize] = each_index as i8;

                initial_index += 1;
            }   

        }

        for each_vecotr in input_vector {

            print!("{} ",each_vecotr);

        }

    }

    pub fn run_prog() {

        let mut string_of_input = String::new();

        let mut no_of_data_points: usize = 0;
        std::io::stdin().read_line(&mut string_of_input);

        no_of_data_points = string_of_input.trim().parse::<usize>().expect("Wrong number");

        let mut input_vector: Vec<i8>;

        string_of_input = String::new();

        std::io::stdin().read_line(&mut string_of_input);

        input_vector = string_of_input.trim().split(" ").into_iter().map(|X| X.trim().parse::<i8>().unwrap()).collect();

        let mut bucket_array: [i8; 3] = [0; 3];

        sort012(&mut input_vector, &mut bucket_array);        

    }

}