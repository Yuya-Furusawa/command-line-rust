use clap::{App, Arg};

fn main() {
    // コマンドライン引数のパース
    let matches = App::new("echor")
        .version("0.1.0")
        .author("hogehoge <test@example.com>")
        .about("Rust echo command")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_new_line")
                .short("n")
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();
    // 入力されたテキストを取得
    let text = matches.values_of_lossy("text").unwrap();
    // 改行を省略するかどうか
    let omit_new_line = matches.is_present("omit_new_line");
    print!(
        "{}{}",
        text.join(" "),
        if omit_new_line { "" } else { "\n" }
    );
}
