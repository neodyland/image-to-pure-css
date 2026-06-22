pub struct ColorRun {
    pub color: image::Rgb<u8>,
    pub end: usize,
}

pub enum Gradient {
    Solid(image::Rgb<u8>),
    Smooth,
}

fn build_row_runs(runs: &mut Vec<ColorRun>, row: &[image::Rgb<u8>]) {
    runs.clear();
    if row.is_empty() {
        return;
    }
    let mut run_color = row[0];

    for (i, &color) in row.iter().enumerate().skip(1) {
        if color != run_color {
            runs.push(ColorRun {
                color: run_color,
                end: i,
            });
            run_color = color;
        }
    }
    runs.push(ColorRun {
        color: run_color,
        end: row.len(),
    });
}

fn runs_to_hard_gradient(parts: &mut Vec<(image::Rgb<u8>, GradientLinearData)>, runs: &[ColorRun]) {
    let last = runs.len() - 1;
    for (i, run) in runs.iter().enumerate() {
        parts.push((
            run.color,
            if i == 0 {
                GradientLinearData::A(run.end)
            } else if i == last {
                GradientLinearData::D
            } else {
                GradientLinearData::B(run.end)
            },
        ));
    }
}

fn can_linear_approximate(
    row: &[image::Rgb<u8>],
    start: usize,
    end: usize,
    tolerance: f32,
) -> bool {
    if end - start <= 1 {
        return true;
    }
    let span = 1.0 / (end - start) as f32;
    let [r, g, b] = row[start].0;
    let mut r = r as f32;
    let mut g = g as f32;
    let mut b = b as f32;
    let [er, eg, eb] = row[end].0;
    let er = (er as f32 - r) * span;
    let eg = (eg as f32 - g) * span;
    let eb = (eb as f32 - b) * span;

    for item in row.iter().take(end).skip(start + 1) {
        r += er;
        g += eg;
        b += eb;
        let [lr, lg, lb] = item.0;
        if (r - lr as f32).abs() > tolerance
            || (g - lg as f32).abs() > tolerance
            || (b - lb as f32).abs() > tolerance
        {
            return false;
        }
    }

    true
}

fn find_optimal_stops(stops: &mut Vec<usize>, row: &[image::Rgb<u8>], tolerance: f32) {
    stops.clear();
    let n = row.len();
    if n <= 2 {
        if n == 1 {
            stops.push(0);
        } else {
            stops.push(0);
            stops.push(n - 1);
        };
        return;
    }
    stops.push(0);
    let mut seg_start = 0;

    while seg_start < n - 1 {
        let mut lo = seg_start + 1;
        let mut hi = n - 1;
        let mut best = seg_start + 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if can_linear_approximate(row, seg_start, mid, tolerance) {
                best = mid;
                lo = mid + 1;
            } else {
                hi = mid.saturating_sub(1);
            }
        }

        if best == n - 1 {
            if stops[stops.len() - 1] != n - 1 {
                stops.push(n - 1);
            }
            break;
        }

        stops.push(best);
        seg_start = best;
    }
}

pub enum GradientLinearData {
    A(usize),
    B(usize),
    C,
    D,
}

fn stops_to_smooth_gradient(
    parts: &mut Vec<(image::Rgb<u8>, GradientLinearData)>,
    row: &[image::Rgb<u8>],
    stops: &[usize],
) {
    parts.clear();
    let last_idx = row.len() - 1;

    for (i, &pos) in stops.iter().enumerate() {
        parts.push((
            row[pos],
            if (i == 0 && pos == 0) || (i == stops.len() - 1 && pos == last_idx) {
                GradientLinearData::C
            } else {
                GradientLinearData::A(pos)
            },
        ));
    }
}

pub fn build_row_gradient_tolerancezero(
    runs: &mut Vec<ColorRun>,
    parts: &mut Vec<(image::Rgb<u8>, GradientLinearData)>,
    row: &[image::Rgb<u8>],
) -> Gradient {
    build_row_runs(runs, row);
    if runs.len() == 1 {
        Gradient::Solid(runs[0].color)
    } else {
        runs_to_hard_gradient(parts, runs);
        Gradient::Smooth
    }
}

pub fn build_row_gradient(
    stops: &mut Vec<usize>,
    parts: &mut Vec<(image::Rgb<u8>, GradientLinearData)>,
    row: &[image::Rgb<u8>],
    tolerance: f32,
) -> Gradient {
    find_optimal_stops(stops, row, tolerance);
    let first_color = row[stops[0]];
    if stops.iter().all(|&s| row[s] == first_color) {
        Gradient::Solid(first_color)
    } else {
        stops_to_smooth_gradient(parts, row, stops);
        Gradient::Smooth
    }
}
