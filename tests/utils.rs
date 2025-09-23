use server::config::Config;

#[cfg(test)]
#[test]
fn load_test() {
    use crate::Config;
    let cfg: Config = utils::load("server", Some("conf/server.toml".to_string()));
    print!("{:?}", cfg);
}
