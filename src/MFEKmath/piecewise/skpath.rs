use super::super::vector::Vector;
use super::super::bezier::Bezier;
use super::super::piecewise::Piecewise;
use skia_safe::{path, Path};

impl Piecewise<Piecewise<Bezier>>
{
    pub fn to_skpath(self) -> Path {
        let path = Path::new();
        return self.append_to_skpath(path);
    }

    pub fn append_to_skpath(&self, mut skpath: Path) -> Path {
        for contour in &self.curves {
            skpath = contour.append_to_skpath(skpath);
        }

        return skpath;
    }
}

impl Piecewise<Bezier>
{
    pub fn append_to_skpath(&self, mut skpath: Path) -> Path
    {
        let mut first = true;
        for bez in &self.curves {
            let controlp = bez.to_control_points();

            if first {
                skpath.move_to(controlp[0].to_skia_point());
                first = false;
            }
            
            // we've got ourselves a line
            if controlp[0] == controlp[2] && controlp[1] == controlp[3] {
                skpath.line_to(controlp[3].to_skia_point());
            }

            skpath.cubic_to(controlp[1].to_skia_point(), controlp[2].to_skia_point(), controlp[3].to_skia_point());
        }

        return skpath;
    }
}

impl From<&Path> for Piecewise<Piecewise<Bezier>>
{
    fn from(ipath: &Path) -> Self {
        let mut contours: Vec<Piecewise<Bezier>> = Vec::new();
        let iter = path::Iter::new(ipath, false);
    
        let mut cur_contour: Vec<Bezier> = Vec::new();
        let mut last_point: Vector = Vector{x: 0., y: 0.}; // don't think we need this?
        for (v, vp) in iter {
            match v {
                path::Verb::Move => {
                    if !cur_contour.is_empty() {
                        contours.push(Piecewise { curves: cur_contour })
                    }
    
                    cur_contour = Vec::new();  
                    last_point = Vector::from_skia_point(vp.first().unwrap());
                }
    
                path::Verb::Line => {
                    let lp = Vector::from_skia_point(&vp[0]);
                    let np = Vector::from_skia_point(&vp[1]);
                    cur_contour.push(Bezier::from_control_points(lp, lp, np, np));
                    last_point = np;
                }
    
                path::Verb::Quad => {
                    let lp = last_point;
                    let h2 = Vector::from_skia_point(&vp[0]);
                    let np = Vector::from_skia_point(&vp[1]);
                    cur_contour.push(Bezier::from_control_points(lp, lp, h2, np));
                    last_point = np;
                }
    
                path::Verb::Cubic => {
                    let lp = Vector::from_skia_point(&vp[0]);
                    let h1 = Vector::from_skia_point(&vp[1]);
                    let h2 = Vector::from_skia_point(&vp[2]);
                    let np = Vector::from_skia_point(&vp[3]);
                    cur_contour.push(Bezier::from_control_points(lp, h1, h2, np));
                    last_point = np;
                }
    
                path::Verb::Close => {
                    contours.push(Piecewise { curves: cur_contour.clone()});
                    cur_contour = Vec::new();
                }
                
    
                // I might have to implement more verbs, but at the moment we're just converting
                // from glifparser output and these are all the supported primitives there.
                _ => { println!("{:?} {:?}", v, vp); panic!("Unsupported skia verb in skpath!"); }
            }
        }
    
        if !cur_contour.is_empty() {
            contours.push(Piecewise{ curves: cur_contour });
        }
    
        return Piecewise {
            curves: contours
        }
    }
}