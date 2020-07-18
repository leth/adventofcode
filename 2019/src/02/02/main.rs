fn compute(program: &[i64], noun: i64, verb: i64) -> i64 {
    let mut memory = program.to_owned();
    // memory.copy_from_slice(program);
    memory[1] = noun;
    memory[2] = verb;

    let mut position = 0;
    loop {
        let opcode = memory[position];

        let lhs_addr = memory[position + 1] as usize;
        let rhs_addr = memory[position + 2] as usize;
        let dest_addr = memory[position + 3] as usize;

        let lhs = memory[lhs_addr];
        let rhs = memory[rhs_addr];

        // println!("op {}", opcode);
        let (op_str, result) = match opcode {
            1 =>
            // Add
            {
                ("+", lhs + rhs)
            }
            2 =>
            // Multiply
            {
                ("+", lhs * rhs)
            }
            99 => break,
            x => panic!("unhandled opcode {}", x),
        };
        // println!("{}\t{}\t{}\t=  {}", lhs, op_str, rhs, result);
        // println!("{}\t \t{}\t-> {}", lhs_addr, rhs_addr, dest_addr);
        // println!();
        memory[dest_addr] = result;

        position += 4
    }
    memory[0]
}

fn main() -> () {
    let program = [
        1, 0, 0, 3, //
        1, 1, 2, 3, //
        1, 3, 4, 3, //
        1, 5, 0, 3, //
        2, 1, 13, 19, //
        2, 9, 19, 23, //
        1, 23, 6, 27, //
        1, 13, 27, 31, //
        1, 31, 10, 35, //
        1, 9, 35, 39, //
        1, 39, 9, 43, //
        2, 6, 43, 47, //
        1, 47, 5, 51, //
        2, 10, 51, 55, //
        1, 6, 55, 59, //
        2, 13, 59, 63, //
        2, 13, 63, 67, //
        1, 6, 67, 71, //
        1, 71, 5, 75, //
        2, 75, 6, 79, //
        1, 5, 79, 83, //
        1, 83, 6, 87, //
        2, 10, 87, 91, //
        1, 9, 91, 95, //
        1, 6, 95, 99, //
        1, 99, 6, 103, //
        2, 103, 9, 107, //
        2, 107, 10, 111, //
        1, 5, 111, 115, //
        1, 115, 6, 119, //
        2, 6, 119, 123, //
        1, 10, 123, 127, //
        1, 127, 5, 131, //
        1, 131, 2, 135, //
        1, 135, 5, 0,  //
        99, //
        2, 0, 14, 0,
    ];

    for x in 0..100 {
        for y in 0..100 {
            let result = compute(&program, x, y);
            if result == 19690720 {
                println!("f({}, {}) = {}", x, y, result);
                println!("{}", (100 * x) + y);
            }
        }
    }
}
