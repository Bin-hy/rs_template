use rs_template::utils;
use server::config::Config;
#[cfg(test)]
#[test]
fn load_test() {
    let cfg: Config = utils::load("server", Some("conf/server.local.toml".to_string()));
    print!("{:?}", cfg);
}
