#[cfg(feature = "openapi")]
use utoipa::OpenApi;

// 导入utoipa自动生成的路径模块
#[cfg(feature = "openapi")]
use crate::route::hello_handler;

#[cfg(feature = "openapi")]
use crate::route::__path_hello_handler;


#[cfg(feature = "openapi")]
use crate::route::user::UserApi;
// 使用derive宏来创建简单的OpenAPI文档
#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "后端服务API",
        description = "这是服务器的API文档",
        version = "0.0.1"
    ),
    paths(
        hello_handler,
    ),
    nest(
        (path = "/users", api = UserApi)
    )
)]
#[cfg_attr(not(feature = "openapi"), allow(unused))]
pub struct ApiDoc;
