pub mod loess {
    // a painful re-write of the C++ program here:
    // https://github.com/hroest/CppLowess/blob/master/include/CppLowess/Lowess.h

    // disclaimer: It's not 100% right, as I was translating it, but understood almost nothing
    // so the tests done in the repo above ^ do not match what I was getting.
    // however, the result looks reasonable on a plot..?

    // I may go back and annotate however
    // I'm also fairly certain this is the first public rust port of a loess function.

    // TODO: make this code safer!

    fn pow2<T>(x: T) -> T
    where
        T: std::ops::Mul<T, Output = T> + Copy,
    {
        x * x
    }

    fn pow3<T>(x: T) -> T
    where
        T: std::ops::Mul<T, Output = T> + Copy,
    {
        x * x * x
    }

    fn calculate_weights(
        x: &[f64], // may have to be f64
        n: usize,
        current_x: f64,
        use_resid_weights: bool,
        nleft: usize,
        resid_weights: &mut [f64],
        weights: &mut [f64],
        nrt: &mut usize,
        h: f64,
    ) -> bool {
        let mut r: f64;
        let mut j: usize;

        let h9 = 0.999 * h;
        let h1 = 0.001 * h;
        let mut a = 0.0; // sum of weights

        j = nleft;

        loop {
            if j == n {
                break;
            }

            r = (x[j] - current_x).abs();

            if r <= h9 {
                if r > h1 {
                    // small enough for non-zero weight
                    // compute tricube function: ( 1 - (r/h)^3 )^3
                    weights[j] = pow3(1.0 - pow3(r / h));
                } else {
                    weights[j] = 1.0;
                }

                if use_resid_weights {
                    weights[j] = resid_weights[j] * weights[j];
                }

                a += weights[j];
            } else if x[j] > current_x {
                // get out at first zero wt on right
                break;
            }

            j += 1;
        }
        // rightmost pt (may be greater than nright because of ties)
        *nrt = j - 1usize;
        if a <= 0.0 {
            false
        } else {
            // normalize weights (make sum of w[j] == 1)

            let mut index1 = nleft;
            loop {
                if index1 == *nrt {
                    break;
                }
                weights[index1] = weights[index1] / a;
                index1 += 1;
            }
            true
        }
    }

    fn calculate_y_fit(
        x: &[f64],
        y: &[f64],
        current_x: f64,
        n: usize,
        nleft: usize,
        nrt: usize,
        h: f64,
        ys: &mut f64,
        weights: &mut [f64],
    ) {
        let range = x[n - 1] - x[0];

        if h > 0.0 {
            // use linear fit

            // No regression function (e.g. lstsq) is called. Instead a "projection
            // vector" p_i_j is calculated, and y_fit[i] = sum(p_i_j * y[j]) = y_fit[i]
            // for j s.t. x[j] is in the neighborhood of x[i]. p_i_j is a function of
            // the weights, x[i], and its neighbors.
            // To save space, p_i_j is computed in place using the weight vector.

            // find weighted center of x values
            let mut sum_weighted_x = 0.0; // originally variable a
            let mut index1 = nleft;
            loop {
                if index1 == nrt {
                    break;
                }
                sum_weighted_x += weights[index1] * x[index1];
                index1 += 1;
            }

            let mut b = current_x - sum_weighted_x; // originally variable b
            let mut weighted_sqdev = 0.0; // originally variable c

            let mut index2 = nleft;
            loop {
                if index2 == nrt {
                    break;
                }
                weighted_sqdev +=
                    weights[index2] * (x[index2] - sum_weighted_x) * (x[index2] - sum_weighted_x);

                index2 += 1;
            }

            if (weighted_sqdev).sqrt() > 0.001 * range {
                // points are spread out enough to compute slope
                b = b / weighted_sqdev;
                let mut index3 = nleft;
                loop {
                    if index3 == nrt {
                        break;
                    }
                    weights[index3] = weights[index3] * (1.0 + b * (x[index3] - sum_weighted_x));

                    index3 += 1;
                }
            }
        }

        *ys = 0.0;
        let mut index4 = nleft;
        loop {
            if index4 == nrt {
                break;
            }
            *ys += weights[index4] * y[index4];

            index4 += 1;
        }
    }

    fn lowest(
        x: &[f64],
        y: &[f64],
        n: usize,
        current_x: f64, //xs
        ys: &mut f64,
        nleft: usize,
        nright: usize,
        weights: &mut [f64],     // vector w
        use_resid_weights: bool, // userw
        resid_weights: &mut [f64],
    ) -> bool {
        let h: f64;
        let mut nrt: usize = 0; //? // rightmost pt (may be greater than nright because of ties)

        h = (current_x - x[nleft]).max(x[nright] - current_x);

        // Calculate the weights for the regression in this neighborhood.
        // Determine if at least some weights are positive, so a regression
        // is ok.
        let fit_ok = calculate_weights(
            x,
            n,
            current_x,
            use_resid_weights,
            nleft,
            resid_weights,
            weights,
            &mut nrt,
            h,
        );

        if !fit_ok {
            return fit_ok;
        }

        // If it is ok to fit, run the weighted least squares regression
        calculate_y_fit(x, y, current_x, n, nleft, nrt, h, ys, weights);

        return fit_ok;
    }

    fn update_neighbourhood(x: &[f64], n: usize, i: usize, nleft: &mut usize, nright: &mut usize) {
        let mut d1: f64;
        let mut d2: f64;

        while *nright < n - 1 {
            // move nleft, nright to right if radius decreases
            d1 = x[i] - x[*nleft];
            d2 = x[*nright + 1] - x[i];
            // if d1 <= d2 with x[nright+1] == x[nright], lowest fixes
            if d1 <= d2 {
                break;
            }
            // radius will not decrease by move right
            *nleft += 1;
            *nright += 1;
        }
    }

    // not sure at all I transcribed this correctly.
    fn update_indices<'a>(
        x: &[f64],
        n: usize,
        delta: f64,
        i: &'a mut usize,
        last: &'a mut usize,
        ys: &mut [f64],
    ) {
        *last = *i;

        let cut: f64 = x[*last] + delta;

        *i = *last + 1;

        loop {
            if *i == n {
                break;
            }
            if x[*i] > cut {
                break;
            }

            if x[*i] == x[*last] {
                ys[*i] = ys[*last];
                *last = *i;
            }

            *i += 1;
        }

        *last += 1;
        *i -= 1;

        *i = (*last).max(*i);
    }

    fn interpolate_skipped_fits(x: &[f64], i: usize, last: usize, ys: &mut [f64]) {
        let mut alpha: f64;
        let denom = x[i] - x[last];

        let mut j = last; // last + 1;
        loop {
            if j == i {
                break;
            }
            alpha = (x[j] - x[last]) / denom;
            ys[j] = alpha * ys[i] + (1.0 - alpha) * ys[last];

            j += 1;
        }
    }

    fn calculate_residual_weights(n: usize, weights: &[f64], resid_weights: &mut [f64]) {
        let mut r: f64;

        let mut i = 0;
        loop {
            if i == n {
                break;
            }
            resid_weights[i] = weights[i].abs();

            i += 1;
        }

        // this changes the order of the slice. does this matter?
        fn median(numbers: &mut [f64]) -> f64 {
            numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = numbers.len() / 2;
            numbers[mid]
        }

        let cmad = 6.0 * median(resid_weights);
        let c9 = 0.999 * cmad;
        let c1 = 0.001 * cmad;

        let mut i2 = 0;

        loop {
            if i2 == n {
                break;
            }
            r = weights[i2].abs();

            if r <= c1 {
                // near 0, avoid underflow
                resid_weights[i2] = 1.0;
            } else if r > c9 {
                // near 1, avoid underflow
                resid_weights[i2] = 0.0;
            } else {
                resid_weights[i2] = pow2(1.0 - pow2(r / cmad));
            }

            i2 += 1;
        }
    }

    pub fn loess(
        x: &[f64],
        y: &[f64],
        frac: f64, // parameter f
        nsteps: i32,
        delta: f64,
        ys: &mut [f64],
        resid_weights: &mut [f64], // vector rw
        weights: &mut [f64],       // vector res
    ) -> Vec<f64> {
        let mut fit_ok: bool;
        let mut i: usize;
        let mut last: usize;
        let mut nleft: usize;
        let mut nright: usize;
        let ns: usize;

        let n = x.len();
        if n < 2 {
            ys[0] = y[0];
            return ys.to_vec();
        }

        // how many points around estimation point should be used for regression:
        // at least two, at most n points
        let tmp = frac * n as f64;
        //   ns = std::max(std::min(tmp, n), 2);
        let min = tmp.min(n as f64);
        ns = min.max(2.0) as usize;

        // robustness iterations
        let mut iter = 0; // was 1
        loop {
            if iter == nsteps + 1 {
                break ();
            }

            nleft = 0;
            nright = ns - 1;
            last = 0; // was -1
            i = 0;

            while last < n {
                update_neighbourhood(x, n, i, &mut nleft, &mut nright);

                fit_ok = lowest(
                    x,
                    y,
                    n,
                    x[i],
                    &mut ys[i],
                    nleft,
                    nright,
                    weights,
                    iter > 1,
                    resid_weights,
                );

                // if something went wrong during the fit, use y[i] as the
                // fitted value at x[i]
                if !fit_ok {
                    ys[i] = y[i];
                }

                // If we skipped some points (because of how delta was set), go back
                // and fit them by linear interpolation.
                if last < i - 1 {
                    interpolate_skipped_fits(x, i, last, ys);
                }

                // Update the last fit counter to indicate we've now fit this point.
                // Find the next i for which we'll run a regression.
                update_indices(x, n, delta, &mut i, &mut last, ys);
            }

            iter += 1;

            i = 0;

            loop {
                if i == n {
                    break;
                }
                weights[i] = y[i] - ys[i];

                i += 1;
            }

            if iter > nsteps {
                break ();
            }

            calculate_residual_weights(n, weights, resid_weights);
        }

        ys.to_vec()
    }

    pub fn loess_wrapper(x: &[f64], y: &[f64], frac: f64, nsteps: i32, delta: f64) -> Vec<f64> {
        let data_size = x.len();
        let mut resid_weights = vec![0.0; data_size];
        let mut weights = vec![0.0; data_size];
        let mut result = vec![0.0; data_size];

        let y = loess(
            x,
            y,
            frac,
            nsteps,
            delta,
            &mut result,
            &mut resid_weights,
            &mut weights,
        );
        y
    }
}
