use std::io::BufReader;
use std::{fs::File, io::BufWriter};

use xml::reader::{EventReader, XmlEvent};
use xmltree::Element;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

struct XmlElement {}

fn main() {
    let file = File::open("basic_set.svg").unwrap();
    let file = BufReader::new(file);

    let mut names_element = Element::parse(file).unwrap();

    //println!("{:#?}", names_element);
    {
        // get first `name` element
        let mut g = names_element
            .get_mut_child("g")
            .expect("Can't find name element");

        for uses in g.children.iter_mut() {
            if let xmltree::XMLNode::Element(element) = uses {
                if element.name == "use" {
                   
                    let mut symbol = String::new();

                    let mut x_str = "0".to_string();
                    let mut y_str = "0".to_string();                    

                    //println!("{:#?}", element);
                    {
                        if let Some(value) = element.attributes.get("href") {
                          symbol = value.to_string();
                        }
                        if let Some(value) = element.attributes.get("transform") {
                          //  println!("transform = {}", value);
                            let mut x_start: usize = 0;
                            let mut x_end: usize = 0;
                            let mut y_start: usize = 0;
                            let mut y_end: usize = 0;

                            if let Some(index) = value.find("(") {
                                x_start = index + 1
                            };

                            if let Some(index) = value.find(",") {
                                x_end = index;
                                y_start = index + 1;
                            };

                            if let Some(index) = value.find(")") {
                                y_end = index
                            };

                            if x_start >= 0 && x_end > x_start {
                                x_str = value[x_start..x_end].to_string();
                            }
                            if y_start >= 0 && y_end > y_start {
                                y_str = value[y_start..y_end].to_string();
                            }

                            if y_str == "0" {
                              println!("x={}", x_str);
                              println!("y={}", y_str);
                            }
                        };
                    }

                    element.attributes.insert("x".to_string(), x_str);
                    element.attributes.insert("y".to_string(), y_str);
                    element.attributes.insert("xlink:href".to_string(), symbol);

                   element.attributes.remove("href");
                    element.attributes.remove("transform");
                }
            }
        }
        //println!("{:#?}", names_element);
        //name.attributes.insert("suffix".to_owned(), "mr".to_owned());

        let write_file = File::create("basic_set2.svg").unwrap();
        let write_file_buffer = BufWriter::new(write_file);

        names_element.write(write_file_buffer);
    }

    // let parser = EventReader::new(file);
    // let mut depth = 0;
    // for e in parser {
    //     match e {
    //         Ok(XmlEvent::StartElement {
    //             name, attributes, ..
    //         }) => {
    //             if name.local_name == "symbol" {
    //                 println!("{}+{}", indent(depth), name.local_name);

    //                 depth += 1;
    //                 for attribute in attributes.iter() {
    //                     println!(
    //                         "{}->{} = {}",
    //                         indent(depth),
    //                         attribute.name,
    //                         attribute.value
    //                     );
    //                 }
    //             }
    //         }
    //         Ok(XmlEvent::EndElement { name }) => {
    //             if name.local_name == "symbol" {
    //                 depth -= 1;
    //                 println!("{}-{}", indent(depth), name.local_name);
    //             }
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             break;
    //         }
    //         _ => {}
    //     }
    // }
}
