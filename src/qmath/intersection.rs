 /* Not working quite yet, but I don't want to lose the code.
    //https://www.particleincell.com/2013/cubic-line-intersection/
    //Ported this article from js to rust for the following two functions. Left out the line segment checks in the second function
    //because we don't have need for that in this case.
    // I can't get this to work at the moment, I'm going to have to build a real-time visualizer to help me check the line
    // vs the reported intersections.
    #[allow(non_snake_case)]
    fn cubic_roots(a: f64, b: f64, c: f64, d: f64) -> Vec<f64>
    {
        let A = b/a;
        let B = c/a;
        let C = d/a;

        let Q = (3.*B - A.powi(2))/9.;
        let R = (9.*A*B - 27.*C - 2.*A.powi(3))/54.;
        let D = Q.powi(3) + R.powi(2); // the polynomical discriminant

        let mut t: [f64; 3] = [0., 0., 0.];

        if D >= 0. 
        { 
            // complex or duplicate roots
            let S = f64::signum(R + f64::sqrt(D)) * f64::powf(f64::abs(R + f64::sqrt(D)), 1./3.);
            let T = f64::signum(R - f64::sqrt(D)) * f64::powf(f64::abs(R - f64::sqrt(D)), 1./3.);

            t[0] =  -A/3. + (S + T);
            t[1] = -A/3. - (S + T)/2.;
            t[2] = -A/3. - (S + T)/2.;
            let Im = f64::abs(f64::sqrt(3.)*(S - T)/2.);

            if Im != 0.0
            {
            t[1] = -1.;
            t[2] = -1.; 
            }
        }
        else
        {
            //distinct real roots
            let th = f64::acos(R/f64::sqrt(f64::powi(Q, 3)));
            t[0] = 2. * f64::sqrt(-Q) * f64::cos(th/3.) - A/3.;
            t[1] = 2. * f64::sqrt(-Q) * f64::cos((th + 2.*std::f64::consts::PI)/3.) - A/3.;
            t[2] = 2. * f64::sqrt(-Q) * f64::cos((th + 2.*std::f64::consts::PI)/3.) - A/3.;
        }

        let mut result = Vec::new();
        for &r in &t
        {
            if !(r<0.0 || r>1.0) && !r.is_nan()
            {
                result.push(r)
            }
        }

        if result.len() > 1 { result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater)); }

        return result;
    }

    // Compute intersections for beziers vs a line defined by two points. Returns a vec of t values at intersections.
    #[allow(non_snake_case)]
    fn intersect_line(&self, line: (Vector, Vector)) -> Vec<f64>
    {
        let l0 = line.0;
        let l1 = line.1;

        let A = l1.y - l0.y;
        let B = l0.x - l1.x;
        let C = l0.x * (l0.y - l1.y) +
                l0.y * (l1.x - l0.x);

        let a = A * self.D + B * self.H;
        let b = A * self.C + B * self.G;
        let c = A * self.B + B * self.F;
        let d = A * self.A + B * self.E + C;

        let r = Bezier::cubic_roots(a, b, c, d);
        
        return r;
    }

    // Returns up to 3 bezier curves from the reuslt 
    fn split_along_line(&self, line: (Vector, Vector)) -> Vec<Bezier>
    {
        let mut output = Vec::new();
        let mut intersections = self.intersect_line(line);

        let mut right = self.clone();
        let mut last_t = 0.;

        // we didn't find any collisions so we just return a clone of ourself
        if intersections.len() == 0 {
            output.push(right);
            return output;
        } 

        while intersections.len() > 0
        {
            let cur_intersection = intersections.remove(0);
            let u = map_values(cur_intersection, last_t, 1., 0., 1.);

            let subdivisions = right.subdivide(u);
            output.push(subdivisions.0);

            last_t = cur_intersection;
            right = subdivisions.1;
        }

        output.push(right);
        return output;
    }


    // from piecewise<piecewise<bezier>>
    #[allow(dead_code)]
    fn split_at_line_intersection(&self, line: (Vector, Vector)) -> Self
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.split_at_line_intersection(line));
        }

        return Piecewise{
            curves: output,
        };
    }

    // from piecewise<bezier>
    fn split_at_line_intersection(&self, line: (Vector, Vector)) -> Piecewise<Bezier>
    {
        let mut new_curves = Vec::new();
        for bez in &self.curves {
            let subdivisions = bez.split_along_line(line);

            for sub in subdivisions {
                new_curves.push(sub);
            }
        }

        return Piecewise {
            curves: new_curves
        }
    }
    
    // Remap values from one range to another. ex: (0.2:0.6) -> (0.0:1.0)
    // used mainly for remapping t-values when splitting a bezier more than once
    fn map_values(v: f64, l0: f64, h0: f64, l1: f64, h1: f64) -> f64
    {
        return l1 + (v - l0) * (h1 - l1) / (h0 - l0);
    }

*/