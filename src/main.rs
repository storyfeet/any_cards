use card_format::Card as FCard;
use clap_conf::prelude::*;
use handlebars::Handlebars;

pub struct GCard<'a> {
    tplate: &'a Handlebars<'a>,
    card: FCard,
}

fn main() -> Result<(), failure::Error> {
    let clp = clap_app!(any_cards =>
        (about:"Makes any kind of svg card using handlebars templates")
        (version:crate_version!())
        (author:"Matthew Stoodley")
        (@arg config:-c +takes_value "Location of config file")
        (@arg template: -t + takes_value "Location of template file")
        (@arg files: -f + takes_value ... "location of files for cards")
    )
    .get_matches();

    let cfg = with_toml_env(&clp, &["any_conf.toml"]);

    let tplate = Handlebars::new();

    //TODO Read template in

    let mut all_cards = Vec::new();
    for fname in cfg.grab_multi().arg("files").req()? {
        let mut f = std::fs::File::open(fname)?;
        let loaded_cards = card_format::load_cards(&mut f)?;
        for c in loaded_cards {
            all_cards.push(GCard {
                tplate: &tplate,
                card: c,
            });
        }
    }

    Ok(())
}
