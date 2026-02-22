/// Largest-Triangle-Three-Buckets (LTTB) downsampling algorithm.
///
/// Reduces a dataset to `target` points while preserving visual shape.
/// Returns indices into the original arrays.
pub fn lttb_downsample(x: &[f64], y: &[f64], target: usize) -> Vec<usize> {
    let len = x.len().min(y.len());

    if target >= len || target < 3 {
        return (0..len).collect();
    }

    let mut sampled = Vec::with_capacity(target);

    // Always include first point
    sampled.push(0);

    let bucket_size = (len - 2) as f64 / (target - 2) as f64;

    let mut prev_selected = 0usize;

    for i in 0..(target - 2) {
        // Current bucket range
        let bucket_start = ((i as f64 * bucket_size) as usize + 1).min(len - 1);
        let bucket_end = (((i + 1) as f64 * bucket_size) as usize + 1).min(len);

        // Next bucket average (for triangle area calculation)
        let next_start = bucket_end;
        let next_end = (((i + 2) as f64 * bucket_size) as usize + 1).min(len);
        let next_count = (next_end - next_start).max(1);
        let avg_x: f64 =
            (next_start..next_end.min(len)).map(|j| x[j]).sum::<f64>() / next_count as f64;
        let avg_y: f64 =
            (next_start..next_end.min(len)).map(|j| y[j]).sum::<f64>() / next_count as f64;

        // Find the point in the current bucket with the largest triangle area
        let mut max_area = -1.0f64;
        let mut max_idx = bucket_start;

        let px = x[prev_selected];
        let py = y[prev_selected];

        for j in bucket_start..bucket_end.min(len) {
            // Triangle area using the cross product formula
            let area = ((px - avg_x) * (y[j] - py) - (px - x[j]) * (avg_y - py)).abs();
            if area > max_area {
                max_area = area;
                max_idx = j;
            }
        }

        sampled.push(max_idx);
        prev_selected = max_idx;
    }

    // Always include last point
    sampled.push(len - 1);

    sampled
}

/// Apply LTTB downsampling to paired x,y data if the dataset is large
/// relative to the target pixel width.
///
/// Returns (downsampled_x, downsampled_y) or clones if no downsampling needed.
pub fn maybe_downsample(x: &[f64], y: &[f64], pixel_width: usize) -> (Vec<f64>, Vec<f64>) {
    let threshold = pixel_width * 2;
    let len = x.len().min(y.len());

    if len <= threshold {
        return (x[..len].to_vec(), y[..len].to_vec());
    }

    let indices = lttb_downsample(x, y, threshold);
    let dx: Vec<f64> = indices.iter().map(|&i| x[i]).collect();
    let dy: Vec<f64> = indices.iter().map(|&i| y[i]).collect();
    (dx, dy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_data_unchanged() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 1.0, 0.0];
        let indices = lttb_downsample(&x, &y, 10);
        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn downsample_preserves_endpoints() {
        let n = 1000;
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| v.sin()).collect();
        let indices = lttb_downsample(&x, &y, 50);
        assert_eq!(*indices.first().unwrap(), 0);
        assert_eq!(*indices.last().unwrap(), n - 1);
        assert_eq!(indices.len(), 50);
    }

    #[test]
    fn downsample_reduces_size() {
        let n = 10000;
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| v.sin()).collect();
        let (dx, dy) = maybe_downsample(&x, &y, 100);
        assert!(dx.len() < n);
        assert_eq!(dx.len(), dy.len());
    }

    #[test]
    fn no_downsample_when_small() {
        let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y = vec![0.0, 1.0, 0.0, 1.0, 0.0];
        let (dx, dy) = maybe_downsample(&x, &y, 100);
        assert_eq!(dx.len(), 5);
        assert_eq!(dy.len(), 5);
    }

    #[test]
    fn indices_are_sorted() {
        let n = 500;
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| (v * 0.1).sin() * 100.0).collect();
        let indices = lttb_downsample(&x, &y, 30);
        for w in indices.windows(2) {
            assert!(w[0] < w[1], "indices should be monotonically increasing");
        }
    }
}
