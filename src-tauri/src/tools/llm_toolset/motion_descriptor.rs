use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MotionPoint {
    x: f64,
    y: f64,
    timestamp: u64,
}

#[derive(Debug)]
struct MotionAnalysis {
    total_distance: f64,
    duration_ms: u64,
    average_speed: f64,
    direction: String,
    shape: String,
}

#[derive(Debug, Serialize, specta::Type)]
pub struct MotionDescriptorResult {
    pub en: String,
    pub ko: String,
}

fn analyze_motion(points: &[MotionPoint]) -> MotionAnalysis {
    if points.len() < 2 {
        return MotionAnalysis {
            total_distance: 0.0,
            duration_ms: 0,
            average_speed: 0.0,
            direction: "none".to_string(),
            shape: "point".to_string(),
        };
    }

    // Calculate total distance
    let mut total_distance = 0.0;
    for i in 1..points.len() {
        let dx = points[i].x - points[i - 1].x;
        let dy = points[i].y - points[i - 1].y;
        total_distance += (dx * dx + dy * dy).sqrt();
    }

    // Duration
    let duration_ms = points.last().unwrap().timestamp - points.first().unwrap().timestamp;

    // Average speed (pixels per second)
    let average_speed = if duration_ms > 0 {
        (total_distance / duration_ms as f64) * 1000.0
    } else {
        0.0
    };

    // Direction (start to end)
    let start = points.first().unwrap();
    let end = points.last().unwrap();
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let direction = if dx.abs() > dy.abs() {
        if dx > 0.0 {
            "right"
        } else {
            "left"
        }
    } else if dy > 0.0 {
        "down"
    } else {
        "up"
    }
    .to_string();

    // Detect shape
    let shape = detect_shape(points, dx, dy);

    MotionAnalysis {
        total_distance,
        duration_ms,
        average_speed,
        direction,
        shape,
    }
}

fn detect_shape(points: &[MotionPoint], dx: f64, dy: f64) -> String {
    // Simple shape detection
    let displacement = (dx * dx + dy * dy).sqrt();
    let mut total_distance = 0.0;

    for i in 1..points.len() {
        let pdx = points[i].x - points[i - 1].x;
        let pdy = points[i].y - points[i - 1].y;
        total_distance += (pdx * pdx + pdy * pdy).sqrt();
    }

    // If displacement is much smaller than total distance, it's a curve
    let ratio = displacement / total_distance;

    if ratio < 0.3 {
        "circular/curved path".to_string()
    } else if ratio > 0.8 {
        "straight line".to_string()
    } else {
        "curved motion".to_string()
    }
}

fn generate_descriptor_text(analysis: &MotionAnalysis, points: &[MotionPoint], sample_count: u32) -> String {
    let speed_desc = if analysis.average_speed > 300.0 {
        "fast"
    } else if analysis.average_speed > 100.0 {
        "moderate"
    } else {
        "slow"
    };

    let duration_sec = analysis.duration_ms as f64 / 1000.0;

    let start = points.first().unwrap();
    let end = points.last().unwrap();
    let mid_idx = points.len() / 2;
    let mid = &points[mid_idx];

    let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

    let total_duration = end.timestamp as f64;

    let normalized_points: Vec<String> = points
        .iter()
        .step_by((points.len() / sample_count as usize).max(1))
        .map(|p| {
            let nx = if max_x > min_x {
                (p.x - min_x) / (max_x - min_x)
            } else {
                0.5
            };
            let ny = if max_y > min_y {
                (p.y - min_y) / (max_y - min_y)
            } else {
                0.5
            };
            let nt = if total_duration > 0.0 {
                p.timestamp as f64 / total_duration
            } else {
                0.0
            };
            format!("pos:({:.2},{:.2}) t:{:.2}", nx, ny, nt)
        })
        .collect();

    let speeds: Vec<f64> = (1..points.len())
        .map(|i| {
            let dx = points[i].x - points[i - 1].x;
            let dy = points[i].y - points[i - 1].y;
            let dt = (points[i].timestamp - points[i - 1].timestamp) as f64;
            let distance = (dx * dx + dy * dy).sqrt();
            if dt > 0.0 {
                distance / dt
            } else {
                0.0
            }
        })
        .collect();

    let max_speed = speeds.iter().copied().fold(0.0, f64::max);
    let min_speed = speeds.iter().copied().fold(f64::INFINITY, f64::min);
    let speed_variation = if max_speed > 0.0 {
        (max_speed - min_speed) / max_speed * 100.0
    } else {
        0.0
    };

    let acceleration_pattern = if speeds.len() > 2 {
        let first_half_avg =
            speeds[..speeds.len() / 2].iter().sum::<f64>() / (speeds.len() / 2) as f64;
        let second_half_avg = speeds[speeds.len() / 2..].iter().sum::<f64>()
            / (speeds.len() - speeds.len() / 2) as f64;

        if second_half_avg > first_half_avg * 1.2 {
            "accelerating"
        } else if first_half_avg > second_half_avg * 1.2 {
            "decelerating"
        } else {
            "constant"
        }
    } else {
        "constant"
    };

    let sampled_text = format!("sampled ~{} points", sample_count);

    format!(
        "Motion Descriptor for LLM:\n\n\
        Visual Representation:\n\
        - The motion path is visualized with speed-based coloring\n\
        - Blue segments: Low speed (minimal movement)\n\
        - Red segments: High speed (rapid movement)\n\
        - P1 (Green dot): Starting point\n\
        - P2 (Red dot): Ending point\n\n\
        Movement Summary:\n\
        - Shape: {}\n\
        - Direction: {}\n\
        - Speed: {} ({:.0} px/s)\n\
        - Duration: {:.2} seconds\n\
        - Total Distance: {:.0} pixels\n\
        - Points Recorded: {}\n\
        - Speed Variation: {:.1}%\n\
        - Acceleration Pattern: {}\n\n\
        Key Points:\n\
        - Start (P1): ({:.1}, {:.1}) at t=0ms\n\
        - Middle: ({:.1}, {:.1}) at t={}ms\n\
        - End (P2): ({:.1}, {:.1}) at t={}ms\n\n\
        Normalized Path Data ({sampled_text}):\n\
        Format: pos:(x,y) t:elapsed_time\n\
        - Position (x,y): 0-1 normalized coordinates\n\
        - Time (t): 0-1 normalized elapsed time (0=start, 1=end)\n\
        Path: {}\n\n\
        Natural Language Description:\n\
        \"A {} {} motion moving {}, covering {:.0} pixels over {:.2} seconds. \
        The motion shows {} speed pattern with {:.1}% speed variation.\"\n\n\
        Prompt Suggestion:\n\
        \"Create an animation that follows a {} path, moving {} at a {} pace for approximately {:.1} seconds. \
        The motion should have a {} acceleration pattern.\"",
        analysis.shape,
        analysis.direction,
        speed_desc,
        analysis.average_speed,
        duration_sec,
        analysis.total_distance,
        points.len(),
        speed_variation,
        acceleration_pattern,
        start.x, start.y,
        mid.x, mid.y, mid.timestamp,
        end.x, end.y, end.timestamp,
        normalized_points.join(" -> "),
        speed_desc,
        analysis.shape,
        analysis.direction,
        analysis.total_distance,
        duration_sec,
        acceleration_pattern,
        speed_variation,
        analysis.shape,
        analysis.direction,
        speed_desc,
        duration_sec,
        acceleration_pattern,
        sampled_text = sampled_text
    )
}

fn generate_descriptor_text_ko(analysis: &MotionAnalysis, points: &[MotionPoint], sample_count: u32) -> String {
    let speed_desc = if analysis.average_speed > 300.0 {
        "빠름"
    } else if analysis.average_speed > 100.0 {
        "보통"
    } else {
        "느림"
    };

    let duration_sec = analysis.duration_ms as f64 / 1000.0;

    let start = points.first().unwrap();
    let end = points.last().unwrap();
    let mid_idx = points.len() / 2;
    let mid = &points[mid_idx];

    let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

    let total_duration = end.timestamp as f64;

    let normalized_points: Vec<String> = points
        .iter()
        .step_by((points.len() / sample_count as usize).max(1))
        .map(|p| {
            let nx = if max_x > min_x { (p.x - min_x) / (max_x - min_x) } else { 0.5 };
            let ny = if max_y > min_y { (p.y - min_y) / (max_y - min_y) } else { 0.5 };
            let nt = if total_duration > 0.0 { p.timestamp as f64 / total_duration } else { 0.0 };
            format!("위치:({:.2},{:.2}) t:{:.2}", nx, ny, nt)
        })
        .collect();

    let speeds: Vec<f64> = (1..points.len())
        .map(|i| {
            let dx = points[i].x - points[i - 1].x;
            let dy = points[i].y - points[i - 1].y;
            let dt = (points[i].timestamp - points[i - 1].timestamp) as f64;
            let distance = (dx * dx + dy * dy).sqrt();
            if dt > 0.0 { distance / dt } else { 0.0 }
        })
        .collect();

    let max_speed = speeds.iter().copied().fold(0.0, f64::max);
    let min_speed = speeds.iter().copied().fold(f64::INFINITY, f64::min);
    let speed_variation = if max_speed > 0.0 {
        (max_speed - min_speed) / max_speed * 100.0
    } else {
        0.0
    };

    let acceleration_pattern = if speeds.len() > 2 {
        let first_half_avg = speeds[..speeds.len() / 2].iter().sum::<f64>() / (speeds.len() / 2) as f64;
        let second_half_avg = speeds[speeds.len() / 2..].iter().sum::<f64>() / (speeds.len() - speeds.len() / 2) as f64;
        if second_half_avg > first_half_avg * 1.2 { "가속" }
        else if first_half_avg > second_half_avg * 1.2 { "감속" }
        else { "일정" }
    } else {
        "일정"
    };

    let direction_ko = match analysis.direction.as_str() {
        "right" => "오른쪽",
        "left" => "왼쪽",
        "up" => "위",
        "down" => "아래",
        _ => "없음",
    };

    let shape_ko = match analysis.shape.as_str() {
        "circular/curved path" => "원형/곡선 경로",
        "straight line" => "직선",
        "curved motion" => "곡선 움직임",
        "point" => "점",
        _ => &analysis.shape,
    };

    format!(
        "LLM용 모션 설명자:\n\n\
        시각적 표현:\n\
        - 모션 경로는 속도 기반 색상으로 시각화됩니다\n\
        - 파란색 구간: 느린 속도\n\
        - 빨간색 구간: 빠른 속도\n\
        - P1 (녹색 점): 시작점\n\
        - P2 (빨간 점): 끝점\n\n\
        움직임 요약:\n\
        - 형태: {}\n\
        - 방향: {}\n\
        - 속도: {} ({:.0} px/s)\n\
        - 지속 시간: {:.2}초\n\
        - 총 이동 거리: {:.0} 픽셀\n\
        - 기록된 포인트: {}개\n\
        - 속도 변동: {:.1}%\n\
        - 가속 패턴: {}\n\n\
        주요 포인트:\n\
        - 시작 (P1): ({:.1}, {:.1}) t=0ms\n\
        - 중간: ({:.1}, {:.1}) t={}ms\n\
        - 끝 (P2): ({:.1}, {:.1}) t={}ms\n\n\
        정규화 경로 데이터 (샘플 ~{}개):\n\
        형식: 위치:(x,y) t:경과시간\n\
        - 위치 (x,y): 0-1 정규화 좌표\n\
        - 시간 (t): 0-1 정규화 경과 시간 (0=시작, 1=끝)\n\
        경로: {}\n\n\
        자연어 설명:\n\
        \"{} {} 움직임이 {} 방향으로, {:.0} 픽셀을 {:.2}초에 걸쳐 이동합니다. \
        {} 속도 패턴을 보이며 속도 변동은 {:.1}%입니다.\"\n\n\
        프롬프트 제안:\n\
        \"{} 경로를 따라 {} 방향으로 {}게 약 {:.1}초간 이동하는 애니메이션을 만드세요. \
        움직임은 {} 가속 패턴을 가져야 합니다.\"",
        shape_ko,
        direction_ko,
        speed_desc,
        analysis.average_speed,
        duration_sec,
        analysis.total_distance,
        points.len(),
        speed_variation,
        acceleration_pattern,
        start.x, start.y,
        mid.x, mid.y, mid.timestamp,
        end.x, end.y, end.timestamp,
        sample_count,
        normalized_points.join(" -> "),
        speed_desc,
        shape_ko,
        direction_ko,
        analysis.total_distance,
        duration_sec,
        acceleration_pattern,
        speed_variation,
        shape_ko,
        direction_ko,
        speed_desc,
        duration_sec,
        acceleration_pattern
    )
}

#[tauri::command]
#[specta::specta]
pub fn generate_motion_descriptor(motion_data: String, sample_count: u32) -> Result<MotionDescriptorResult, String> {
    // Parse JSON
    let points: Vec<MotionPoint> =
        serde_json::from_str(&motion_data).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    if points.is_empty() {
        return Err("No motion data provided".to_string());
    }

    // Analyze motion
    let analysis = analyze_motion(&points);

    // Generate descriptor
    Ok(MotionDescriptorResult {
        en: generate_descriptor_text(&analysis, &points, sample_count),
        ko: generate_descriptor_text_ko(&analysis, &points, sample_count),
    })
}
