mod go_temp;
use clap_conf::prelude::*;
use failure::Fail;
//use gtmpl::Context;
use gtmpl::Template;
use gtmpl_helpers::THelper;
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

#[derive(Fail, Debug)]
#[fail(display = "String Err{}", 0)]
pub struct StrErr(String);
impl From<String> for StrErr {
    fn from(s: String) -> Self {
        StrErr(s)
    }
}

impl StrErr {
    pub fn ext(mut self, s: &str) -> Self {
        self.0.push_str(s);
        self
    }
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
        (@arg c_width:-w --card_width + takes_value "Card width")
        (@arg c_height:-h --card_height + takes_value "Card width")
        (@arg n_width:-a +takes_value "Num Cards across per page")
        (@arg n_height:-d +takes_value "Num Cards down per page")
        (@arg margin: --margin +takes_value"Margin size")
    )
    .get_matches();

    let cfg = with_toml_env(&clp, &["any_conf.toml"]);

    let mut template = Template::default().with_defaults().with_exec();

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
    template.parse(tfs).map_err(|e| StrErr(e))?;

    let mut all_cards = Vec::new();
    for fname in cfg.grab_multi().arg("files").conf("files").req()? {
        let mut f = std::fs::File::open(&fname).map_err(|e| q_err(&format!("{:?}", fname), e))?;
        let loaded_cards = card_format::load_cards(&mut f)
            .map_err(|e| q_err(&format!("file = {:?}", fname), e))?;
        all_cards.extend(loaded_cards);
    }

    let obase = cfg.grab().arg("out_base").conf("out_base").def("out/");
    let c_width: Option<f64> = cfg.grab().arg("cwidth").conf("card.width").t_done();
    let c_height: Option<f64> = cfg.grab().arg("cheight").conf("card.height").t_done();
    let g_width: Option<usize> = cfg.grab().arg("n_width").conf("grid.width").t_done();
    let g_height: Option<usize> = cfg.grab().arg("n_height").conf("grid.height").t_done();

    let margin: Option<f64> = cfg.grab().arg("margin").conf("page.margin").t_done();

    let mut f_base = obase.clone();
    f_base.push_str("f_");
    let mut svb = mksvg::page::Pages::build();
    if let (Some(w), Some(h)) = (c_width, c_height) {
        println!("Setting card size = ({},{})", w, h);
        svb = svb.card_size(w, h);
    }
    if let (Some(w), Some(h)) = (g_width, g_height) {
        println!("Setting grid size = ({},{})", w, h);
        svb = svb.grid_size(w, h);
    }
    if let Some(m) = margin {
        println!("Setting margin = {}", m);
        svb = svb.with_margin(m);
    }

    println!("Preparing cards");
    let (_, _pages) = svb.write_pages(
        f_base,
        &mut mksvg::iter::spread(&all_cards, |c| c.num),
        &|wr, w, h, c| -> Result<(), StrErr> {
            let cw = go_temp::CWH::new(&c.name, w, h, &c.data);
            let rs = template
                .q_render(cw)
                .map_err(|e| format!("{} on card {}:{:?}", e, c.name, c.data))?;
            wr.write(&rs);
            Ok(())
        },
    )?;

    Ok(())
}
