use ploot::prelude::*;

fn main() {
    // Simulated 20-day stock price data
    let days: Vec<f64> = (1..=20).map(|i| i as f64).collect();
    let mut open  = Vec::new();
    let mut high  = Vec::new();
    let mut low   = Vec::new();
    let mut close = Vec::new();

    let mut price = 100.0;
    for i in 0..20 {
        let t = i as f64;
        let o = price;
        let change = (t * 1.3).sin() * 3.0 + (t * 0.7).cos() * 2.0;
        let c = o + change;
        let h = o.max(c) + (t * 2.1).sin().abs() * 2.0 + 0.5;
        let l = o.min(c) - (t * 1.7).cos().abs() * 2.0 - 0.5;
        open.push(o);
        high.push(h);
        low.push(l);
        close.push(c);
        price = c;
    }

    let candles = CandlestickPlot::new(
        days.iter().copied(),
        open.iter().copied(),
        high.iter().copied(),
        low.iter().copied(),
        close.iter().copied(),
    )
    .with_caption("ACME Corp");

    let layout = Layout2D::new()
        .with_title("Candlestick Chart")
        .with_x_label("day")
        .with_y_label("price ($)")
        .with_plot(candles);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
