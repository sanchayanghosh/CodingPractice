pub mod prog8 {


    fn cyclically_rotate_array(array_1: &mut Vec<i32>, size_of_array: usize) {

        let mut last_variable: i32 = array_1[size_of_array-1];

        let mut present_variable: i32 = 0;
        let mut next_variable: i32 = array_1[0];


        for index in 0..size_of_array-1 {

            present_variable = array_1[index+1];

            array_1[index+1] = next_variable;

            next_variable = present_variable;

        }

        array_1[0] = next_variable;



    } 

    pub fn run_prog() {

        let mut array: Vec<i32> = vec![2, 3, 4];

        cyclically_rotate_array(&mut array, 3);   

        for each_arr in array {
            print!("{}, ", each_arr);
        }     

    }

}