// You are given a string s and two integers x and y. You can perform two types of operations any number of times.

// Remove substring "ab" and gain x points:
//  - For example, when removing "ab" from "cabxbae" it becomes "cxbae".
// Remove substring "ba" and gain y points:
//  - For example, when removing "ba" from "cabxbae" it becomes "cabxe".
// Return the maximum points you can gain after applying the above operations on s.

pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let (letter, opposite, h, l) = if x >= y {
        ('b', 'a', x, y)
    } else {
        ('a', 'b', y, x)
    };
    let mut stack: Vec<char> = Vec::new();
    let mut score = 0i32;
    let mut i: usize;

    for c in s.chars() {
        if c == opposite {
            stack.push(c);
        } else if c == letter {
            if stack.len() >= 1 && stack[stack.len() - 1] != letter {
                stack.pop();
                score += h;
            } else {
                stack.push(c);
            }
        } else {
            i = 0;
            while stack.len() > 1 && i < stack.len() {
                if stack[i] == letter && stack[stack.len() - 1] == opposite {
                    i += 1;
                    score += l;
                }
                stack.pop();
            }
            stack.clear();
        }
    }

    i = 0;
    while stack.len() > 1 && i < stack.len() {
        if stack[i] == letter && stack[stack.len() - 1] == opposite {
            i += 1;
            score += l;
        }
        stack.pop();
    }

    score
}

fn main() {
    // first example
    let s1 = "aabbabkbbbfvybssbtaobaaaabataaadabbbmakgabbaoapbbbbobaabvqhbbzbbkapabaavb\
        beghacabamdpaaqbqabbjbababmbakbaabajabasaabbwabrbbaabbafubayaazbbbaababbaaha";
    let (x1, y1) = (1926, 4320);

    // Second example
    let s2 = "cdbcbbaaabab";
    let (x2, y2) = (4, 5);

    // Third example
    let s3 = "aabbaaxybbaabb";
    let (x3, y3) = (5, 4);

    println!("{}", maximum_gain(s1.to_owned(), x1, y1));
    println!("{}", maximum_gain(s2.to_owned(), x2, y2));
    println!("{}", maximum_gain(s3.to_owned(), x3, y3));
}
