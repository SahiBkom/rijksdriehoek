// The city Amsterfoort is used as reference Rijksdriehoek coordinate.
const REF_RD_X: f32 = 155000.0;
const REF_RD_Y: f32 = 463000.0;

// The city Amsterfoort is used as reference WGS84 coordinate.
const REF_WGS_84_X: f32 = 52.15517;
const REF_WGS_84_Y: f32 = 5.387206;

/// Convert wgs84 to rijksdriehoek in meters
pub fn wgs84_to_rijksdriehoek<N: num_traits::Float>(
    wgs84_lattitude: N,
    wgs84_longitude: N,
) -> (N, N) {
    // The city Amsterfoort is used as reference Rijksdriehoek coordinate.
    let ref_rd_x = N::from(REF_RD_X).unwrap();
    let ref_rd_y = N::from(REF_RD_Y).unwrap();

    // The city Amsterfoort is used as reference WGS84 coordinate.
    let ref_wgs_84_x = N::from(REF_WGS_84_X).unwrap();
    let ref_wgs_84_y = N::from(REF_WGS_84_Y).unwrap();

    const R_PQ: [[f32; 5]; 4] = [
        [0.0, 190094.945, -0.008, -32.391, 0.0],
        [-0.705, -11832.228, 0.0, -0.608, 0.0],
        [0.0, -114.221, 0.0, 0.148, 0.0],
        [0.0, -2.340, 0.0, 0.0, 0.0],
    ];

    const S_PQ: [[f32; 5]; 4] = [
        [0.0, 0.433, 3638.893, 0.0, 0.092],
        [309056.544, -0.032, -157.984, 0.0, -0.054],
        [73.077, 0.0, -6.439, 0.0, 0.0],
        [59.788, 0.0, 0.0, 0.0, 0.0],
    ];

    let d_lattitude = N::from(0.36).unwrap() * (wgs84_lattitude - ref_wgs_84_x);
    let d_longitude = N::from(0.36).unwrap() * (wgs84_longitude - ref_wgs_84_y);

    let mut calc_latt: N = N::zero();
    let mut calc_long: N = N::zero();

    for p in 0..4 {
        for q in 0..5 {
            calc_latt = calc_latt
                + N::from(R_PQ[p][q]).unwrap()
                    * d_lattitude.powi(p as i32)
                    * d_longitude.powi(q as i32);
            calc_long = calc_long
                + N::from(S_PQ[p][q]).unwrap()
                    * d_lattitude.powi(p as i32)
                    * d_longitude.powi(q as i32);
        }
    }

    ((ref_rd_x + calc_latt), (ref_rd_y + calc_long))
}

pub fn rijksdriehoek_to_wgs84<N: num_traits::Float>(x: N, y: N) -> (N, N) {
    // The city Amsterfoort is used as reference Rijksdriehoek coordinate.
    let ref_rd_x = N::from(REF_RD_X).unwrap();
    let ref_rd_y = N::from(REF_RD_Y).unwrap();

    // The city Amsterfoort is used as reference WGS84 coordinate.
    let ref_wgs_84_x = N::from(REF_WGS_84_X).unwrap();
    let ref_wgs_84_y = N::from(REF_WGS_84_Y).unwrap();

    let dx = (x - ref_rd_x) * N::from(0.00001).unwrap();
    let dy = (y - ref_rd_y) * N::from(0.00001).unwrap();

    let vec_p_q_k = vec![
        (0, 1, 3235.65389),
        (2, 0, -32.58297),
        (0, 2, -0.24750),
        (2, 1, -0.84978),
        (0, 3, -0.06550),
        (2, 2, -0.01709),
        (1, 0, -0.00738),
        (4, 0, 0.00530),
        (2, 3, -0.00039),
        (4, 1, 0.00033),
        (1, 1, -0.00012),
    ];

    let vec_p_q_l = vec![
        (1, 0, 5260.52916),
        (1, 1, 105.94684),
        (1, 2, 2.45656),
        (3, 0, -0.81885),
        (1, 3, 0.05594),
        (3, 1, -0.05607),
        (0, 1, 0.01199),
        (3, 2, -0.00256),
        (1, 4, 0.00128),
        (0, 2, 0.00022),
        (2, 0, -0.00022),
        (5, 0, 0.00026),
    ];

    let mut sx = N::zero();
    let mut sy = N::zero();

    for e in vec_p_q_k {
        sx = sx + N::from(e.2).unwrap() * dx.powi(e.0) * dy.powi(e.1);
    }

    for e in vec_p_q_l {
        sy = sy + N::from(e.2).unwrap() * dx.powi(e.0) * dy.powi(e.1);
    }

    (
        ref_wgs_84_x + sx / N::from(3600).unwrap(),
        ref_wgs_84_y + sy / N::from(3600).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_f32(a: f32, b: f32, delta: f32) {
        assert!((a - b).abs() < delta, "{} != {}", a, b);
    }

    #[test]
    fn amsterdam_westertoren_to_rd() {
        let (x, y): (f32, f32) = wgs84_to_rijksdriehoek(52.37453253, 4.88352559);
        assert_f32(x, 120700.723, 0.5);
        assert_f32(y, 487525.502, 0.5);
    }

    #[test]
    fn amsterdam_westertoren_to_wgs84() {
        let (x, y): (f32, f32) = rijksdriehoek_to_wgs84(120700.723, 487525.502);
        assert_f32(x, 52.37453253, 0.00001);
        assert_f32(y, 4.88352559, 0.00001);
    }

    #[test]
    fn groningen_martinitoren_to_rd() {
        let (x, y): (f64, f64) = wgs84_to_rijksdriehoek(53.21938317, 6.56820053);
        assert!((x - 233883.131).abs() < 0.5, "{} != {}", x, 233883.131);
        assert!((y - 582065.168).abs() < 0.5, "{} != {}", y, 582065.168);
    }
}
