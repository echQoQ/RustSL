// 时间检测：通过调用拼多多API获取服务器时间，休眠300秒后再次获取，判断本地休眠是否被加速
#[cfg(feature = "vm_check_time")]
pub fn check_time() -> bool {
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    // 获取本地时间戳1
    let local_time1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    // 调用拼多多API获取服务器时间戳1
    let api_time1 = match get_pinduoduo_time() {
        Some(t) => t,
        None => return false,
    };

    // 休眠300秒
    thread::sleep(Duration::from_secs(300));

    // 获取本地时间戳2
    let local_time2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    // 调用拼多多API获取服务器时间戳2
    let api_time2 = match get_pinduoduo_time() {
        Some(t) => t,
        None => return false,
    };

    // 判断差值：如果API时间差值大于290秒（考虑网络延迟），则认为本地休眠未被加速
    let api_diff = api_time2 - api_time1;
    let local_diff = local_time2 - local_time1;

    // 如果本地差值远小于API差值，说明被加速
    if local_diff < api_diff - 10 { // 允许10秒误差
        return false; // 检测到沙箱
    }

    true
}

fn get_pinduoduo_time() -> Option<i64> {
    let url = "http://api.pinduoduo.com/api/server/_stm";
    match minreq::get(url).send() {
        Ok(response) => {
            if response.status_code == 200 {
                let body = response.as_str().unwrap_or("");
                // 提取数字时间戳
                let mut time_str = String::new();
                for c in body.chars() {
                    if c.is_ascii_digit() {
                        time_str.push(c);
                    }
                }
                time_str.parse::<i64>().ok()
            } else {
                None
            }
        }
        Err(_) => None,
    }
}