use glifparser::*;
extern crate xmlwriter;
use xmlwriter::*;

fn point_type_to_string(ptype: PointType) -> Option<String>
{
    return match ptype{
        PointType::Undefined => None,
        PointType::OffCurve => None,
        PointType::QClose => None, // should probably be removed from PointType
        PointType::Move => Some(String::from("move")),
        PointType::Curve => Some(String::from("curve")),
        PointType::QCurve => Some(String::from("qcurve")),
        PointType::Line => Some(String::from("line")),
    }
}

fn write_ufo_point_from_handle(mut writer: XmlWriter, handle: Handle) -> XmlWriter
{
    match handle {
        Handle::At(x, y) => {
            writer.start_element("point");
                writer.write_attribute("x", &x);
                writer.write_attribute("y", &y);
            writer.end_element();
        },
        _ => {}
    }

    return writer;
}

pub fn write_ufo_glif<T>(glif: Glif<T>) -> String
{
    let mut writer = XmlWriter::new(Options::default());

    writer.start_element("glyph");
    writer.write_attribute("name", &glif.name);
    writer.write_attribute("format", &glif.format);

    writer.start_element("advance");
    writer.write_attribute("width", &glif.width);
    writer.end_element();

    match glif.unicode
    {
        Codepoint::Hex(hex) => {
            writer.start_element("unicode");
            writer.write_attribute("hex", &format!(r#"{:X}"#, hex as u32));
            writer.end_element();
        },
        Codepoint::Undefined => {}
    }

    match glif.anchors
    {
        Some(anchor_vec) => {
            for anchor in anchor_vec {
                writer.start_element("anchor");
                writer.write_attribute("x", &anchor.x);
                writer.write_attribute("y", &anchor.y);
                writer.write_attribute("name", &anchor.class);
                // Anchor does not currently contain a color, or identifier attribute
                writer.end_element();
            }
        },
        None => {}
    }

    match glif.outline
    {
        Some(outline) => {
            writer.start_element("outline");
        
            // if we find a move point at the start of things we set this to false

            for contour in outline {
                let open_contour = if contour.first().unwrap().ptype == PointType::Move { true } else { false };


                writer.start_element("contour");
                
                let mut last_point = None;
                for point in &contour {
                    if let Some(lp) = last_point {
                        // if there was a point prior to this one we emit our b handle
                        writer = write_ufo_point_from_handle(writer, point.b);
                    }


                
                    writer.start_element("point");
                        writer.write_attribute("x", &point.x);
                        writer.write_attribute("y", &point.y);
                
                        match point_type_to_string(point.ptype) {
                            Some(ptype_string) => writer.write_attribute("type", &ptype_string),
                            None => {}
                        }
                
                        match &point.name {
                            Some(name) => writer.write_attribute("name", &name),
                            None => {}
                        }
                
                        // Point>T> does not contain fields for smooth, or identifier.
                    writer.end_element();
                
                    match point.ptype {
                        PointType::Line | PointType::Curve => {
                            writer = write_ufo_point_from_handle(writer, point.a);
                        },

                        PointType::QCurve => {
                            //QCurve currently unhandled. This needs to be implemented.
                        },
                        _ => { } // I don't think this should be reachable in a well formed Glif object?
                    }    
                    
                    last_point = Some(point);
                }

                // if a move wasn't our first point then we gotta close the shape by emitting the first point's b handle
                if !open_contour {
                    writer = write_ufo_point_from_handle(writer, contour.first().unwrap().b);
                }

                writer.end_element();
            }
            writer.end_element();
        },
        None => {}
    }

    writer.end_document()
}