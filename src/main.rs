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

    fn pop(&mut self) -> String {
        self.stack.pop().unwrap()
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

    let start_index = 0;
    let end_index = 23; // end index is exclusive
    loop_through_test_cases(start_index, end_index, json_file, &mut stack)

}

#[derive(Debug, PartialEq, Eq)]
enum OpcodeResult {
    Continue,
    Stop,
    Jump(usize)
}

fn loop_through_test_cases(start_index: usize, end_index: usize, json_file: Value, stack: &mut Stack) {

    for instruction_index in start_index..end_index {
        // retrieve the instruction from the json file at the index of instruction_index
        let instruction = json_file.as_array().unwrap()[instruction_index]["code"]["asm"].as_str().unwrap();
        let instruction_array = instruction.split_whitespace().collect::<Vec<&str>>();
        println!("The instruction is {:?}", instruction_array);
    
        // loop through the instruction array and process the opcodes. Increment by two because the opcodes and their values are in pairs
        let mut i: usize = 0;
        while (i < instruction_array.len()) {
            let opcode = *instruction_array.get(i ).unwrap_or(&"");
            let value = *instruction_array.get(i + 1).unwrap_or(&""); // could be out of bounds

            println!("Processing opcode {opcode} with value {value}", opcode = opcode, value = value);
        

            let opcode_result = process_opcode(opcode, value, stack); // so far everything but stop opcode is true

            match opcode_result {
                OpcodeResult::Stop => {
                    break;
                },
                OpcodeResult::Continue => {
                    println!("Continue");
                },
                OpcodeResult::Jump(jump_index) => {
                    println!("Jumping to index {jump_index}", jump_index = jump_index);
                    i = jump_index;
                },
            }
        
            
            // index increment is based off the opcode
            if opcode.starts_with("PUSH") {
                println!("Incrementing by 2");
                i += 2;
            } else {
                println!("Incrementing by 1");

                i += 1;
            }
        }
      
    
        // retrieve the stack value from the json file
        let expected_stack = &json_file.as_array().unwrap()[instruction_index]["expect"]["stack"];
        let expected_stack_vector = expected_stack.as_array().unwrap();


         // Define a closure to square each element
        let convert_to_string: fn(&Value) -> String  = |x| String::from(&from_value::<String>(x.clone()).unwrap()[2..]);

         // Call the `map` function on the vector, passing the closure
        let transformed_vec = expected_stack_vector.iter().map(convert_to_string).collect::<Vec<_>>();

        // expected stack is in reverse order and transformed_vec is in correct order, for debugging purposes
        println!("The expected stack is {:?}", transformed_vec);
        println!("The actual stack is {:?}", stack.stack.iter().rev().collect::<Vec<_>>());

        // check stacks are equal
        assert_eq!(transformed_vec.iter().eq(stack.stack.iter().rev()), true);

        // stacks equal
        println!("Stacks are equal!");

        // reset stack
        *stack = Stack::new();
    }
}
 

fn process_opcode(opcode: &str, value: &str, stack: &mut Stack) -> OpcodeResult {
    match opcode {
        "STOP" => {
            println!("We are stopping");
            return OpcodeResult::Stop;
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
            stack.push(value);
        },
        "PUSH6" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },
        "PUSH10" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },
        "PUSH11" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },
        "PUSH32" => {
            let value = String::from(&value[2..]);
            stack.push(value);
        },
        "POP" => {
            stack.pop();
        },
        "ADD" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();

            let wrapping_add_val = val1.wrapping_add(val2).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);

        },
        "MUL" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();

            let wrapping_add_val = val1.wrapping_mul(val2).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);

        },
        "SUB" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();

            let wrapping_add_val = val1.wrapping_sub(val2).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);

        },
        "DIV" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();

            if (val2 == 0) {
                stack.push(String::from("0"));
                return OpcodeResult::Continue;
            }

            let wrapping_add_val = val1.wrapping_div(val2).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);

        },
        "MOD" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();
            if (val2 == 0) {
                stack.push(String::from("0"));
                return OpcodeResult::Continue;
            }

            let wrapping_add_val = (val1 % val2).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);

        },
        "ADDMOD" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
            let val3: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();
            let val3 = u32::from_str_radix(&val3, 16).unwrap();

            if (val2 == 0) {
                stack.push(String::from("0"));
                return OpcodeResult::Continue;
            }

            let wrapping_add_val = ((val1 + val2) % val3).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);
        },
        "MULMOD" => {
            let val1: String = stack.pop();
            let val2: String = stack.pop();
            let val3: String = stack.pop();
    
            let val1 = u32::from_str_radix(&val1, 16).unwrap_or_default();
            let val2 = u32::from_str_radix(&val2, 16).unwrap();
            let val3 = u32::from_str_radix(&val3, 16).unwrap();

            if (val2 == 0) {
                stack.push(String::from("0"));
                return OpcodeResult::Continue;
            }

            let wrapping_add_val = ((val1 * val2) % val3).to_string();
            let added = String::from(wrapping_add_val);

            stack.push(added);
        },
        "JUMP" => {
            let jump_dest: String = stack.pop();
            println!("Jumping to {jump_dest}");
            return OpcodeResult::Jump(jump_dest.trim().parse::<usize>().unwrap());
        }
        _ => {
            println!("No opcodes matched, opcode we got was {opcode}")
        }
    };
    return OpcodeResult::Continue;
}