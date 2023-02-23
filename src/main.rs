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


    let instruction = json_file.as_array().unwrap()[2]["code"]["asm"].as_str().unwrap();
    let instruction_array = instruction.split_whitespace().collect::<Vec<&str>>();
    println!("The instruction is {:?}", instruction_array);

    for i in (0..instruction_array.len()).step_by(2) {
        process_opcode(instruction_array[i], instruction_array[i+1], &mut stack)
    }

    let expected_stack_vector = &json_file.as_array().unwrap()[2]["expect"]["stack"];
    let mut expected_stack = expected_stack_vector.get(0).unwrap().clone();
    let expected_stack = from_value::<String>(expected_stack).unwrap();
    let expected_stack = &expected_stack[2..];
    println!("Expected Stack is {expected_stack}");

    let binding = stack.stack[0].clone();
    let current_stack = binding.as_str();
    let current_stack = current_stack;

    println!("The current stack is {current_stack}");
    assert_eq!(current_stack, expected_stack);

}


fn process_opcode(opcode: &str, value: &str, stack: &mut Stack) {
    match opcode {
        "PUSH2" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },
        _ => {
            println!("No opcodes matched")
        }
    };
}