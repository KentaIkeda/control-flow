fn main() {
    let n = 3;

    // Rustのif文は、式を（）で囲まず、そのまま記述する
    if n < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rustのif文の式は、完全にboolであるべき
    // JavaScriptの様に、暗黙的に値がboolに変換されることはない。
    // このコードは、nが3ですが、trueと評価されず、コンパイルエラーが発生する
    // if n {
    //     println!("{}", n);
    // }

    {
        const N: i32 = 6;

        let is_divisible_2: bool = is_divisible_n_by_x(N, 2);
        let is_divisible_3: bool = is_divisible_n_by_x(N, 3);
        let is_divisible_4: bool = is_divisible_n_by_x(N, 4);

        if !is_divisible_4 && !is_divisible_3  && !is_divisible_2  {
            println!("number is not divisible by 4, 3, or 2");
            // 本来はエラーをスローさせたほうが良い
            return;
        }

        if is_divisible_4 {
            // 数値は4で割り切れます
            println!("number is divisible by 4");
        } else if is_divisible_3 {
            // 数値は3で割り切れます
            println!("number is divisible by 3");
        } else if is_divisible_2 {
            // 数値は2で割り切れます
            println!("number is divisible by 2");
        }
    }
}

fn is_divisible_n_by_x(n: i32, x: i32) -> bool {
    n % x == 0
}
