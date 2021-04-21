use ::sxd_document::dom::{ChildOfElement, ChildOfRoot};
use ::sxd_document::parser;

#[test]
fn it_works() {
    let package = parser::parse(super::POM).unwrap();
    for i in package.as_document().root().children() {
        if let ChildOfRoot::Element(e) = i {
            if e.name().local_part().ne("project") {
                continue;
            }
            for i in e.children() {
                if let ChildOfElement::Element(e) = i {
                    if e.name().local_part().ne("dependencies") {
                        continue;
                    }
                    for i in e.children() {
                        if let ChildOfElement::Element(e) = i {
                            if e.name().local_part().ne("dependency") {
                                continue;
                            }
                            let (mut g, mut a, mut v) =
                                (String::new(), String::new(), String::new());
                            for i in e.children() {
                                if let ChildOfElement::Element(e) = i {
                                    for i in e.children() {
                                        if let ChildOfElement::Text(t) = i {
                                            match e.name().local_part() {
                                                "groupId" => g = t.text().to_string(),
                                                "artifactId" => a = t.text().to_string(),
                                                "version" => v = t.text().to_string(),
                                                _ => (),
                                            }
                                        }
                                    }
                                }
                            }
                            println!("{}:{}:{}", g, a, v);
                        }
                    }
                    break;
                }
            }
        }
    }
}
