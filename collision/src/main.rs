use std::io;

fn number_reader(info: &str, limit: bool) -> i64 {
    loop {
        println!("{}", info);
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("讀取錯誤");

        let number:i64 = match temp.trim().parse() {
            Ok(number) => {
                number
            }
            Err(err) => {
                println!("{}",err);
                continue
            }
        };

        if limit {
            if number <= 0 {
                println!("輸入值須大於等於0");
                continue
            }
            return number;
        }
        return number;
    }
}

fn main() {
    println!("彈性碰撞轉換器");
    println!("輸入只支援整數");
    let weight_left = number_reader("請輸入左側物體質量", true);
    let speed_left = number_reader("請輸入左側物體速度", false);

    let weight_right = number_reader("請輸入右側物體質量", true);
    let speed_right = number_reader("請輸入右側物體速度", false);


    if speed_left < speed_right {
        println!("你的物體不會撞在一起");
        return;
    }

    let v1 = ((weight_left - weight_right) * speed_left + 2 * weight_right * speed_right) as f64 / (weight_right + weight_left) as f64;
    let v2 = ((weight_right - weight_left) * speed_right + 2 * weight_left * speed_left) as f64 / (weight_right + weight_left) as f64;

    println!("左邊的速度: {}", v1);
    println!("右邊的速度: {}", v2);
}
