fn main() {
    let price = 129;
    let tax = 20.34;
    let total = f64::from( price) + tax;
    println!("Total: {} + {} = {}", price, tax, total);
    let spidermans = ["Muguire","Holland"];//array definition + instantiation
    println!(" array: {:?}",spidermans);

    let mut top_scores = [0; 3];
    top_scores[0] = 292;
    top_scores[1] = 170;
    top_scores[2] = 140;
    let [my_score, _, _] = top_scores;
    println!("score: {:?}",my_score);

    //tuples
    let mut product = ("iPhone 12 Pro Max", 1099, true);
    product = ("PS5", 499, false);
    println!("tuple:{:#?}",product)
}
