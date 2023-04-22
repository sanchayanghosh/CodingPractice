pub mod prog1 {


pub fn run_prog() {

    let mut data_strings = String::new();

    std::io::stdin().read_line(&mut data_strings).expect("Wrong input");


    let size_of_input: usize = data_strings.trim().parse::<usize>().unwrap();
    
    data_strings = String::new();

    std::io::stdin().read_line(&mut data_strings).expect("Enter string of integers by spaces");


    for each_index in 0..size_of_input/2 {
        
        let temp_val = data_strings.chars().nth(each_index).unwrap();

        data_strings.insert(each_index, data_strings.chars().nth(size_of_input-1-each_index).unwrap());

        data_strings.remove(each_index+1);

        data_strings.insert(size_of_input-1-each_index, temp_val);

        data_strings.remove(size_of_input-each_index);

    }

    print!("{}",data_strings);


}

}