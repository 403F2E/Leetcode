#[allow(unused_assignments)]
fn flower_game(n: i32, m: i32) -> i64 {
    let mut n_odd: i32 = n / 2;
    let mut m_odd: i32 = m / 2;

    println!("n_even : {}, m_even: {}", n / 2, m / 2);
    println!("n_odd : {}, m_odd : {}", n_odd, m_odd);

    n_odd = if (n_odd % 2) != 0 { n_odd + 1 } else { n_odd };
    m_odd = if (m_odd % 2) != 0 { m_odd + 1 } else { m_odd };

    println!("n_even : {}, m_even: {}", n / 2, m / 2);
    println!("n_odd : {}, m_odd : {}", n_odd, m_odd);

    ((n_odd * (m / 2)) + ((n / 2) * m_odd)) as i64
}

fn main() {
    println!("{}", flower_game(3, 2));
}
