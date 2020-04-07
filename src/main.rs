use clap_conf::prelude::*;

fn main() -> Result<(), ConfError> {
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

    for v in cfg.grab_multi().arg("files").req()? {
        println!("File = {:?}", v)
    }

    Ok(())
}
