pub mod prog1 {

pub fn run_prog() {

    let mut data_strings = String::new();

    std::io::stdin().read_line(&mut data_strings).expect("Wrong input");


    let size_of_input: usize = data_strings.trim().parse::<usize>().unwrap();
    
    data_strings = String::new();

    std::io::stdin().read_line(&mut data_strings).expect("Enter string of integers by spaces");

    let input_types = data_strings.trim().split(" ").into_iter();
    let mut array_input: Vec<i8> = input_types.map(|x| x.trim().parse::<i8>().expect("Wrong error")).collect();

    let mut temp_val;

    for each_index in 0..size_of_input/2 {
        
        temp_val = array_input[each_index];

        
        array_input[each_index] = array_input[size_of_input - 1 - each_index];

        array_input[size_of_input - 1 - each_index] = temp_val;

    }

    for each_input in array_input {

        print!("{} ", each_input);

    }


}

}