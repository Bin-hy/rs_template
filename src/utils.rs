use log::error;

/// 加载 配置 文件

pub fn load<T>(file_name: &str, path: Option<String>) -> T
where
    T: serde::de::DeserializeOwned + std::default::Default,
{
    use ::std::fs::read_to_string;
    // 如果给出了path读取path的文件 ，若未给出则
    // 读取 {name}.toml , 若读不到则
    // 读取 /etc/rs_template/{name}.toml, 也读不到的话
    // 返回 ""
    let result = read_to_string(path.unwrap_or(format!("{file_name}.toml")))
        .or(read_to_string(format!("/etc/rs_template/{file_name}.toml")))
        .unwrap_or("".to_string());

    // 返回配置
    toml::from_str(result.as_str()).unwrap_or_else(|e| {
        error!("config load error :{}", e);
        Default::default()
    })
}
