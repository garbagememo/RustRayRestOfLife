use getopts::Options;
use std::process;

// 引数を格納する構造体
#[derive(Debug)]
pub struct Args {
    //   repeat: usize,
    pub s: usize,
    pub w: usize,
    pub m: usize,
    pub output: String,
}

fn print_usage(exe_name: &str, opts: &Options) {
    let brief = format!("Usage: {}  [Options]", exe_name);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

pub fn parameters() -> Args {
    // コマンドラインオプションを取得
    let args: Vec<String> = std::env::args().collect();

    // キーワード引数を指定
    let mut opts = Options::new();
    opts.optopt("s", "samples", "sampling number", "1..etc");
    opts.optopt("w", "width", "screen width", "ex)768");
    opts.optopt("m", "model", "model number", "0..9");
    opts.optopt("o", "output", "set output file name", "[FILE]");
    opts.optflag("h", "help", "print this help");

    // パース
    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!("{}", f.to_string()));

    // "h" が存在したらヘルプを表示して終了
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
    }

    // 必要な位置引数がなかったらヘルプを表示して終了
    //    if matches.free.is_empty() {print_usage(&args[0], &opts); }

    // キーワード引数の取得
    let sampstr = matches.opt_str("s").unwrap_or("32".to_string());
    let s: usize = sampstr.parse().unwrap();
    let w = matches
        .opt_str("w")
        .unwrap_or("768".to_string())
        .parse()
        .unwrap();
    let m = matches
        .opt_str("m")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
    let output = matches.opt_str("o").unwrap_or("image.png".to_string());
    // 位置引数の取得
    //    let repeat = matches.free[0].clone().parse::<usize>().unwrap_or_else(|f| panic!("{}",f.to_string()));

    // 構造体の生成
    let ret = Args { s, w, m, output };
    return ret;
}
#[allow(dead_code)]
fn test() {
    let args = parameters();
    println!("{:?}", args);
}
