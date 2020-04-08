mod helpers;
use card_format::CData;
//use card_format::Card as FCard;
use clap_conf::prelude::*;
//use handlebars::Handlebars;
//use mksvg::Card as SCard;
//use mksvg::SvgWrite;
use failure::Fail;
use serde_derive::Serialize;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Fail, Debug)]
#[fail(display = "{}{}", s, e)]
pub struct QErr<E: Fail> {
    e: E,
    s: String,
}
pub fn q_err<E: Fail>(s: &str, e: E) -> QErr<E> {
    QErr {
        s: s.to_string(),
        e,
    }
}

#[derive(Serialize)]
struct CWH<'a> {
    name: &'a str,
    w: f64,
    h: f64,
    data: &'a BTreeMap<String, CData>,
}

fn main() -> Result<(), failure::Error> {
    let clp = clap_app!(any_cards =>
        (about:"Makes any kind of svg card using handlebars templates")
        (version:crate_version!())
        (author:"Matthew Stoodley")
        (@arg config:-c +takes_value "Location of config file")
        (@arg template: -t + takes_value "Location of template file (config template)")
        (@arg files: -f + takes_value ... "location of files for cards (config files [...])")
        (@arg out_base: -o +takes_value "Location base for output files")
        (@arg c_width:-w + takes_value "Card width")
        (@arg c_height:-h+ takes_value "Card width")
        (@arg g_width: + takes_value "Card width")
        (@arg g_height: + takes_value "Card width")
    )
    .get_matches();

    let cfg = with_toml_env(&clp, &["any_conf.toml"]);

    let mut tplate = helpers::help_template();

    //TODO Read template in
    let tfname = cfg
        .grab_local()
        .arg("template")
        .conf("template")
        .env("ANY_CARD_TEMPLATE")
        .def("front_temp.svg");
    let mut tfile = std::fs::File::open(&tfname).map_err(|e| q_err(&format!("{:?}", tfname), e))?;
    let mut tfs = String::new();
    tfile.read_to_string(&mut tfs)?;
    tplate.register_template_string("front", tfs)?;

    let mut all_cards = Vec::new();
    for fname in cfg.grab_multi().arg("files").conf("files").req()? {
        let mut f = std::fs::File::open(&fname).map_err(|e| q_err(&format!("{:?}", fname), e))?;
        let loaded_cards = card_format::load_cards(&mut f)?;
        all_cards.extend(loaded_cards);
    }

    let obase = cfg.grab().arg("out_base").conf("out_base").def("out/");
    let c_width: Option<f64> = cfg.grab().arg("c_width").conf("card.width").t_done();
    let c_height: Option<f64> = cfg.grab().arg("c_height").conf("card.height").t_done();
    let g_width: Option<usize> = cfg.grab().arg("g_width").conf("grid.width").t_done();
    let g_height: Option<usize> = cfg.grab().arg("g_height").conf("grid.height").t_done();

    let mut f_base = obase.clone();
    f_base.push_str("f_");
    let mut svb = mksvg::page::Pages::build();
    if let (Some(w), Some(h)) = (c_width, c_height) {
        svb = svb.card_size(w, h);
    }
    if let (Some(w), Some(h)) = (g_width, g_height) {
        svb = svb.grid_size(w, h);
    }
    let (_, _pages) = svb.write_pages(
        f_base,
        &mut mksvg::iter::spread(&all_cards, |c| c.num),
        &|wr, w, h, c| -> Result<(), handlebars::RenderError> {
            let cwh = CWH {
                name: &c.name,
                w,
                h,
                data: &c.data,
            };
            let rs = tplate.render("front", &cwh)?;
            wr.write(&rs);
            Ok(())
        },
    )?;

    Ok(())
}
