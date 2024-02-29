use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+)d(\d+)$").unwrap();
    let input = "2d6";

    let (d, f) = match re.captures(input) {
        // str のままだと df がブロックの外に出た途端に死ぬ．スタックから抜ける．
        // なので .to_string() で別のメモリに割り当てるように？実体化する．
        Some(df) => (df[1].to_string(), df[2].to_string()),
        None => {
            println!("none0.");
            return;
        }
    };

    println!("d={}, f={}.", &d, &f);
}
