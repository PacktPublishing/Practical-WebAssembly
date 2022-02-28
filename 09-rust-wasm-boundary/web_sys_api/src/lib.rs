use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw(percent: i32) -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let div = document.create_element("div")?;
    let ns = Some("http://www.w3.org/2000/svg");

    div.set_attribute("class", "pie")?;

    let svg = document.create_element_ns( ns, "svg")?;
    svg.set_attribute("height", "100")?;
    svg.set_attribute("width", "100")?;
    svg.set_attribute("viewBox", "0 0 32 32")?;

    let circle = document.create_element_ns(ns, "circle")?;
    circle.set_attribute("r", "16")?;
    circle.set_attribute("cx", "16")?;
    circle.set_attribute("cy", "16")?;
    circle.set_attribute("stroke-dasharray", &(percent.to_string().to_owned() +" 100"))?;

    svg.append_child(&circle)?;

    div.append_child(&svg)?;

    Ok(div)
}