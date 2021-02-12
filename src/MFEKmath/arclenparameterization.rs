use super::evaluate::Evaluate;
use super::parameterization::Parameterization;
use super::vector::Vector;
use crate::MFEKmath::coordinate::*;
// We build a table of total arc length along the line and use it to map 0-1
// to the arclength of the curve such that 0.5 is halfway along the curve by arc-length
pub struct ArcLengthParameterization
{
    pub arclens: Vec<f64>
}

impl ArcLengthParameterization
{
    pub fn from<T: Evaluate>(evaluable: &T) -> Self
    {
        let mut output = Vec::new();
        // TODO: this is an arbitrary number and should be replaced with something more robust
        // TODO: preferably a tolerance value
        let arclen_cuts = 10000;
        let max_cuts = 10000 + 1;

        let mut prev_point = evaluable.at(0.0);
        let mut sum = 0.0;
        output.push(sum);
        
        let mut i = 1;
        while i < max_cuts
        {
            let t = i as f64 / arclen_cuts as f64;
            let point = evaluable.at(t);
            let dist = point.distance(prev_point);
            sum = sum + dist;
            output.push(sum);

            prev_point = point;
            i = i + 1;
        }

        return Self {
            arclens: output
        }
    }

    pub fn get_total_arclen(&self) -> f64
    {
        return *self.arclens.last().unwrap();
    }

    // Have to implement a custom binary search here because we're looking
    // not for an exact index but the index of the highest value that's less than
    // the target
    fn search_for_index(&self, target: f64) -> usize
    {
        let mut left = 0;
        let mut right = self.arclens.len() - 1;

        while left < right {
            let middle = (right+left)/2;

            if left == middle { return middle; }
            if right == middle { return left; }
            if self.arclens[middle] == target { return middle };

            if self.arclens[middle] < target {
                left = middle
            }
            else
            {
                right = middle;
            }
        }

        // This needs to be replaced with success/failure.
        panic!("Couldn't find the target arc length!")
    }
}

impl Parameterization for ArcLengthParameterization
{
    fn parameterize(&self, u: f64) -> f64
    {
        let target_arclen = u * self.arclens[self.arclens.len() - 1];
 
        let arclen_index = self.search_for_index(target_arclen);
        if target_arclen == self.arclens[arclen_index]
        {
           return arclen_index as f64 / (self.arclens.len() - 1) as f64;
        }
        else
        {
            let len_start = self.arclens[arclen_index];
            let len_end = self.arclens[arclen_index+1];
            let segment_len = len_end - len_start;

            let segment_fraction = (target_arclen - len_start) / segment_len;

            return (arclen_index as f64 + segment_fraction) / (self.arclens.len() - 1) as f64;
        }
    }
}