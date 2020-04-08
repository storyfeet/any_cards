use handlebars::*;
use mksvg::text::wrap;
use mksvg::text::Text;

handlebars_helper!(mul_f:|a:f64,b:f64|a*b);
handlebars_helper!(wrapl:|s:str,l:u64|{to_json(wrap(s,l as usize))});

pub fn text_fn(s: &str, x: f64, y: f64, lhight: f64, wrap: usize) -> String {
    let t = Text::new(s, x, y, lhight).wrap(wrap).v_center().to_string();
    t
}

pub fn help_template() -> Handlebars<'static> {
    let mut res = Handlebars::new();
    res.register_helper("mul_f", Box::new(mul_f));
    res.register_helper("wrap", Box::new(wrapl));
    res
}
