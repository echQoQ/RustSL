// IP检测：通过ip-api.com获取IP归属地，若非中国则判定为沙箱或代理环境
#[cfg(feature = "vm_check_ip")]
pub fn check_ip() -> bool {
    let url = "http://ip-api.com/csv";
    match minreq::get(url).send() {
        Ok(response) => {
            if response.status_code == 200 {
                let body = response.as_str().unwrap_or("");
                if body.contains("China") {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        Err(_) => false,
    }
}