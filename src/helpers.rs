use handlebars::*;

handlebars_helper!(mul_f:|a:f64,b:f64|a*b);

pub fn help_template() -> Handlebars<'static> {
    let mut res = Handlebars::new();
    res.register_helper("mul_f", Box::new(mul_f));
    res
}
