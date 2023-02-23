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

    fn push<'a>(&mut self, value: String) {
        self.stack.push(value);
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

    let instruction_to_process = 0;

    let instruction = json_file.as_array().unwrap()[instruction_to_process]["code"]["asm"].as_str().unwrap();
    let instruction_array = instruction.split_whitespace().collect::<Vec<&str>>();
    println!("The instruction is {:?}", instruction_array);

    for i in (0..instruction_array.len()).step_by(2) {
        println!("I is {i}");
        process_opcode(instruction_array[i], instruction_array[i+1], &mut stack)
    }

    let expected_stack_vector = &json_file.as_array().unwrap()[instruction_to_process]["expect"]["stack"];
    let expected_stack = expected_stack_vector.get(0).unwrap().clone();
    let expected_stack = &from_value::<String>(expected_stack).unwrap()[2..];
    println!("Expected Stack is {expected_stack}");

    let binding = &stack.stack[0];
    let current_stack = binding.as_str();
    let current_stack = current_stack;

    println!("The current stack is {current_stack}");
    assert_eq!(current_stack, expected_stack);

}


fn loop_through_test_cases(start_index: usize, end_index: usize, json_file: Value) {

    for instruction_index in start_index..end_index {
        let instruction = json_file.as_array().unwrap()[instruction_index]["code"]["asm"].as_str().unwrap();

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
        _ => {
            println!("No opcodes matched")
        }
    };
}