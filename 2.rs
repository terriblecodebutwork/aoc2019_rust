use std::str::FromStr;

fn intcode(code: &Vec<i32>) -> i32 {
    let mut code = code.clone();
    code[1] = 12;
    code[2] = 2;
    let mut i = 0;
    loop {
        let p1 = code[i+1] as usize;
        let p2 = code[i+2] as usize;
        let p3 = code[i+3] as usize;
        match code[i] {
            1 => {
                code[p3] = code[p1] + code[p2];
            },
            2 => {
                code[p3] = code[p1] * code[p2];
            },
            _ => {
                return code[0];
            }
        }
        i += 4;
    }
}


fn main() {
    let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,6,23,2,13,23,27,1,27,13,31,1,9,31,35,1,35,9,39,1,39,5,43,2,6,43,47,1,47,6,51,2,51,9,55,2,55,13,59,1,59,6,63,1,10,63,67,2,67,9,71,2,6,71,75,1,75,5,79,2,79,10,83,1,5,83,87,2,9,87,91,1,5,91,95,2,13,95,99,1,99,10,103,1,103,2,107,1,107,6,0,99,2,14,0,0";
    let code = input
            .split(",")
            .map(|x| i32::from_str(x).unwrap() )
            .collect::<Vec<_>>();
    let x = intcode(&code);
    println!("part1: {:?}", x);
    // println!("part2: {:?}", y);
}
