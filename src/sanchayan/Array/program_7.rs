pub mod prog7 {
    use std::num;


    fn search_for_element_in_range(numbers: &Vec<i32>, low : usize, high: usize, check_number: i32) -> usize {

        if high == 0 {
            return high;
        }

        if low <= high {
            
            if check_number > numbers[(low + high)/2] {
                return search_for_element_in_range(numbers, (low+high)/2+1, high, check_number)
            }
            
            if check_number < numbers[(low + high)/2] {
                return search_for_element_in_range(numbers, low, (low + high)/2-1, check_number)
            }

            return (low + high)/2

        }

        return low


    }

    fn DoUnionAgnostic(number1: Vec<i32>, number2: Vec<i32>, number_1_size: usize, number_2_size: usize) -> Vec<i32> {

        if number_1_size < number_2_size {

            return doUnion(number1, number2, number_1_size,number_2_size
            );

        }

        else {
            return doUnion(number2, number1, number_2_size,number_1_size);
        }

    }


    fn doUnion(number1: Vec<i32>, number2: Vec<i32>, number_1_size: usize,number_2_size: usize) -> Vec<i32> {

        let mut index_returned: usize = 0;

        let mut previous_index: usize = 0;

        let mut union_value: Vec<i32> = vec![];

        let mut should_continue = false;
        let mut union_index: usize = 0;
        let mut each_comparing_no: i32 = 0 ;
        let mut should_omit: bool = false;

            for each_no in number2 {

                index_returned = search_for_element_in_range(&number1, index_returned, number_1_size-1, each_no);

                println!("Step {} is this variable:", union_index);

                each_comparing_no = *number1.get(index_returned).expect("No valid no.");
                

                for each_int in previous_index..index_returned+1 {
                    union_value.push(number1[each_int]);
                    union_index += 1;
                    index_returned += 1;

            }

            

            if each_no != each_comparing_no {

                union_value.push(each_no);
                union_index += 1;

            }

            if previous_index < index_returned {
                previous_index = index_returned;
            }

        }

            if previous_index < number_1_size-1 {

                for each_index in previous_index..number_1_size {

                    union_value.push(number1[each_index]);
    
                }

            }

            


            return union_value;

        }
    

        pub fn run_prog() {


            let vec1 = vec![2, 3, 4];

            let vec2 = vec![3, 5, 6];

            // for each_no in 2..0 {
            //     print!("{}, ", each_no);
            // }

            for each_no in DoUnionAgnostic(vec1, vec2, 3, 3) {

                print!("{}, ", each_no);
                
            }


    

        }
        

    }


    