fn moving_average<T>(data: &[T], window: usize) -> Vec<f64>
where
    T: Copy + Into<f64>,
{
    if window == 0 || window > data.len() {
        return Vec::new();
    }
    data.windows(window)
        .map(|w| w.iter().map(|&x| x.into()).sum::<f64>() / window as f64)
        .collect()
}

fn main() {
    let int_data = [2, 4, 6, 8, 10, 12];
    let float_data = [1.0, 2.5, 4.5, 7.0, 11.0];

    let avg_int = moving_average(&int_data, 3);
    let avg_float = moving_average(&float_data, 2);

    println!("Int averages:   {:?}", avg_int);
    println!("Float averages: {:?}", avg_float);
}
