use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustErr {
    #[error("Something wrong with Request parameter : {0}")]
    ReqParamError(String),

    #[error("Something wrong when Delete : {0}")]
    ReqDeleteFail(String),

    #[error("App rule : {0}")]
    AppRuleError(String)
}