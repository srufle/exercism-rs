pub fn verse(n: u32) -> String {
    match n {
        3..=99 => {
            let first = format!(
                "{} bottles of beer on the wall, {} bottles of beer.",
                &n, &n
            );
            let second = format!(
                "Take one down and pass it around, {} bottles of beer on the wall.",
                &n - 1
            );

            format!("{}\n{}\n", first, second)
        }
        2 => {
            let first = format!(
                "{} bottles of beer on the wall, {} bottles of beer.",
                &n, &n
            );
            let second =
                format!("Take one down and pass it around, 1 bottle of beer on the wall.",);

            format!("{}\n{}\n", first, second)
        }
        1 => {
            let first = format!("{} bottle of beer on the wall, {} bottle of beer.", &n, &n);
            let second =
                format!("Take it down and pass it around, no more bottles of beer on the wall.",);

            format!("{}\n{}\n", first, second)
        }
        0 => {
            let first = format!("No more bottles of beer on the wall, no more bottles of beer.",);
            let second =
                format!("Go to the store and buy some more, 99 bottles of beer on the wall.",);

            format!("{}\n{}\n", first, second)
        }
        _ => {
            // limit program to 99 bottles of beer
            String::from("")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: Vec<String> = vec![];
    println!("start: {:?}, end: {:?}", start, end);
    for n in (end..start + 1).rev() {
        println!("making verse: {:?}", n);
        song.push(verse(n));
        song.push(String::from('\n'));
    }
    println!("{:?}", song);
    let mut ans = song.join("");
    ans.truncate(ans.len() - 1);
    ans
}
