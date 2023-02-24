use std::thread::current;

use serde_json::{Number, Value, from_value, from_str};

struct Stack {
    stack: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: String) {
        self.stack.push(value);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let mut json_file = {

        // Path to the JSON file.
        let mut input_path = "test-cases.json";

        // Load the first file into a string.
        let text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let mut stack = Stack::new();

    let start_index = 2;
    let end_index = 10; // end index is exclusive
    loop_through_test_cases(start_index, end_index, json_file, &mut stack)

}

fn loop_through_test_cases(start_index: usize, end_index: usize, json_file: Value, stack: &mut Stack) {

    for instruction_index in start_index..end_index {
        // retrieve the instruction from the json file at the index of instruction_index
        let instruction = json_file.as_array().unwrap()[instruction_index]["code"]["asm"].as_str().unwrap();
        let instruction_array = instruction.split_whitespace().collect::<Vec<&str>>();
        println!("The instruction is {:?}", instruction_array);
    
        // loop through the instruction array and process the opcodes. Increment by two because the opcodes and their values are in pairs
        for i in (0..instruction_array.len()).step_by(2) {
            let opcode = instruction_array[i];
            let value = instruction_array[i+1];
            
            process_opcode(opcode, value, stack);
        }
    
        // retrieve the stack value from the json file
        let expected_stack = &json_file.as_array().unwrap()[instruction_index]["expect"]["stack"];
        let expected_stack_vector = expected_stack.as_array().unwrap();


         // Define a closure to square each element
        let convert_to_string: fn(&Value) -> String  = |x| String::from(&from_value::<String>(x.clone()).unwrap()[2..]);

         // Call the `map` function on the vector, passing the closure
        let transformed_vec = expected_stack_vector.iter().map(convert_to_string).collect::<Vec<_>>();

        println!("The transformed vector is {:?}", transformed_vec);

        // check stacks are equal
        assert_eq!(transformed_vec.iter().eq(stack.stack.iter().rev()), true);

        // reset stack
        *stack = Stack::new();
    }
}
 

fn process_opcode(opcode: &str, value: &str, stack: &mut Stack) {
    match opcode {
        "STOP" => {
            println!("We are stopping");
        },
        "PUSH1" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },   
        "PUSH2" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },    
        "PUSH4" => {
            let value = String::from(&value[2..]);
            println!("We are pushing to the stack {value}");
            stack.push(value);
        },
        "PUSH6" => {
            let value = String::from(&value[2..]);
            println!("We are pushing to the stack {value}");
            stack.push(value);
        },
        "PUSH10" => {
            let value = String::from(&value[2..]);
            println!("We are pushing to the stack {value}");
            stack.push(value);
        },
        "PUSH11" => {
            let value = String::from(&value[2..]);
            println!("We are pushing to the stack {value}");
            stack.push(value);
        },
        "PUSH32" => {
            let value = String::from(&value[2..]);
            println!("We are pushing to the stack {value}");
            stack.push(value);
        },
        "POP" => {
            stack.pop();
        },
        _ => {
            println!("No opcodes matched")
        }
    };
}