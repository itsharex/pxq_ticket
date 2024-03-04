use thiserror::Error;

#[derive(Error, Debug)]
pub enum PXQError {
    #[error("请求数据错误")]
    ReqwestError,

    #[error("解析数据错误")]
    ParseDataError,

    #[error("文件访问错误")]
    FileAccessError,

    #[error("获取用户信息失败")]
    GetUserProfileError,

    #[error("查询门票信息失败")]
    SearchShowError,

    #[error("查询门票场次信息失败")]
    QueryShowSessionsError,

    #[error("添加抢票提醒失败")]
    AddReminderError,

    #[error("添加抢票提醒失败")]
    TicketWaitlistError,

    #[error("刷新AccessToken失败")]
    RefreshTokenError,

    #[error("获取观演人信息失败")]
    GetUserAudienceError,

    #[error("获取用户地区失败")]
    GetUserLocationError,

    #[error("获取用户地区失败")]
    GetUserAddressError,

    #[error("获取票档失败")]
    GetSeatPlansError,

    #[error("预下单失败")]
    PreOrderError,

    #[error("创建订单失败")]
    CreateOrderError,

    #[error("请在演唱会公布开售时间后再运行!")]
    ShowTimeUnknownError,
}

impl serde::Serialize for PXQError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
