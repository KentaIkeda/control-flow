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

        const IS_DIVISIBLE_2: bool = N % 2 == 0;
        const IS_DIVISIBLE_3: bool = N % 3 == 0;
        const IS_DIVISIBLE_4: bool = N % 4 == 0;

        if !IS_DIVISIBLE_4 && !IS_DIVISIBLE_3  && !IS_DIVISIBLE_2  {
            println!("number is not divisible by 4, 3, or 2");
            // 本来はエラーをスローさせたほうが良い
            return;
        }

        if IS_DIVISIBLE_4 {
            // 数値は4で割り切れます
            println!("number is divisible by 4");
        } else if IS_DIVISIBLE_3 {
            // 数値は3で割り切れます
            println!("number is divisible by 3");
        } else if IS_DIVISIBLE_2 {
            // 数値は2で割り切れます
            println!("number is divisible by 2");
        }
    }
}
