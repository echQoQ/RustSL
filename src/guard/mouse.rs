#[allow(dead_code)]
pub fn has_human_mouse_movement() -> bool {
    use rustcrypt_ct_macros::obf_lit_bytes;
    use std::mem::{size_of, transmute};
    use windows_sys::Win32::Foundation::POINT;
    use crate::utils::{load_library, get_proc_address};

    #[inline]
    fn simple_rand(seed: &mut u32) -> u32 {
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        *seed
    }

    #[inline]
    fn distance(ax: i32, ay: i32, bx: i32, by: i32) -> f64 {
        let dx = (bx - ax) as f64;
        let dy = (by - ay) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    #[repr(C)]
    struct LastInputInfo { cb_size: u32, dw_time: u32 }

    unsafe {
        // 解析需要的 API
        let user32 = match load_library(&obf_lit_bytes!(b"user32.dll\0")) {
            Ok(h) => h,
            Err(_) => return false,
        };
        let kernel32 = match load_library(&obf_lit_bytes!(b"kernel32.dll\0")) {
            Ok(h) => h,
            Err(_) => return false,
        };

        let p_get_cursor_pos = match get_proc_address(user32, &obf_lit_bytes!(b"GetCursorPos\0")) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let get_cursor_pos: unsafe extern "system" fn(*mut POINT) -> i32 = transmute(p_get_cursor_pos);

        let p_get_last_input = get_proc_address(user32, &obf_lit_bytes!(b"GetLastInputInfo\0")).ok();
        let get_last_input: Option<unsafe extern "system" fn(*mut LastInputInfo) -> i32> =
            p_get_last_input.map(|f| transmute(f));

        let p_sleep = match get_proc_address(kernel32, &obf_lit_bytes!(b"Sleep\0")) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let sleep: unsafe extern "system" fn(u32) = transmute(p_sleep);

        let p_tick = get_proc_address(kernel32, &obf_lit_bytes!(b"GetTickCount\0")).ok();
        let get_tick: Option<unsafe extern "system" fn() -> u32> = p_tick.map(|f| transmute(f));

        // 采样参数
        const N: usize = 3; // 采样点数量
        let mut points: [POINT; N] = [POINT { x: 0, y: 0 }; N];
        let mut ticks: [u32; N] = [0; N];

        // 初始种子
        let mut seed = match get_tick { Some(f) => f(), None => 123456789u32 };

        // 第一个点（立即）
        if get_cursor_pos(&mut points[0] as *mut POINT) == 0 { return false; }
        ticks[0] = match get_tick { Some(f) => f(), None => 0 };

        for i in 1..N {
            let jitter = 500u32 + (simple_rand(&mut seed) % 500); // 500~999ms
            sleep(jitter);
            if get_cursor_pos(&mut points[i] as *mut POINT) == 0 { return false; }
            ticks[i] = match get_tick { Some(f) => f(), None => ticks[i-1].wrapping_add(jitter) };
        }

        let mut total_dist = 0f64;
        let net_disp = distance(points[0].x, points[0].y, points[N-1].x, points[N-1].y);
        let mut speeds: [f64; N-1] = [0.0; N-1];
        let mut turns = 0u32;

        for i in 1..N {
            let d = distance(points[i-1].x, points[i-1].y, points[i].x, points[i].y);
            total_dist += d;
            let dt_ms = ticks[i].wrapping_sub(ticks[i-1]) as f64;
            if dt_ms > 0.0 { speeds[i-1] = d / dt_ms; }
        }

        // 统计转角（基于三点夹角），阈值 ~ 12°
        for i in 2..N {
            let v1x = (points[i-1].x - points[i-2].x) as f64;
            let v1y = (points[i-1].y - points[i-2].y) as f64;
            let v2x = (points[i].x - points[i-1].x) as f64;
            let v2y = (points[i].y - points[i-1].y) as f64;
            let n1 = (v1x * v1x + v1y * v1y).sqrt();
            let n2 = (v2x * v2x + v2y * v2y).sqrt();
            if n1 > 0.0 && n2 > 0.0 {
                let dot = v1x * v2x + v1y * v2y;
                let cos_t = (dot / (n1 * n2)).clamp(-1.0, 1.0);
                let angle = cos_t.acos();
                if angle.to_degrees() > 12.0 { turns = turns.saturating_add(1); }
            }
        }

        // 速度离散度（变速是否明显）
        let mut mean_v = 0f64;
        for v in speeds.iter() { mean_v += *v; }
        mean_v /= (N - 1) as f64;
        let mut var_v = 0f64;
        for v in speeds.iter() { let dv = *v - mean_v; var_v += dv * dv; }
        var_v /= (N - 1) as f64;
        let cov_v = if mean_v > 0.0 { (var_v.sqrt() / mean_v).abs() } else { 0.0 };

        let straight_eff = if total_dist > 0.0 { net_disp / total_dist } else { 1.0 };

        let recent_input_bonus = if let Some(glast) = get_last_input {
            let mut lii = LastInputInfo { cb_size: size_of::<LastInputInfo>() as u32, dw_time: 0 };
            if glast(&mut lii as *mut LastInputInfo) != 0 {
                if let Some(gtick) = get_tick {
                    let now = gtick();
                    let delta = now.wrapping_sub(lii.dw_time);
                    (delta as u32) <= 30_000
                } else { false }
            } else { false }
        } else { false };

        // 打分规则（>=2 判定为人类，降低采样点数后相应降低阈值）
        let mut score = 0u32;
        // 1) 路径长度阈值（像素）：过滤微小抖动
        if total_dist >= 30.0 { score += 1; }
        // 2) 存在至少一次明显转向（采样点少时可能无转向）
        if turns >= 1 { score += 1; }
        // 3) 速度波动适中（过于平滑或完全不动都不给分）
        if cov_v >= 0.1 { score += 1; }
        // 4) 不完全直线（允许轻微直线操作，阈值 0.98）
        if straight_eff <= 0.98 { score += 1; }
        // 5) 近期有人机输入迹象
        if recent_input_bonus { score += 1; }

        score >= 2
    }
}