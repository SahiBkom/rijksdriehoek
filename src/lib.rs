pub mod rijksdriehoek {

    pub fn convert_to_rijksdriehoek(wgs84_lattitude: f32, wgs84_longitude: f32) -> (f32, f32){
		// The city Amsterfoort is used as reference Rijksdriehoek coordinate.
        const REF_RD_X: f32 = 155000.0;
        const REF_RD_Y: f32 = 463000.0;

        // The city Amsterfoort is used as reference WGS84 coordinate.
        const REF_WGS_84_X: f32 = 52.15517;
        const REF_WGS_84_Y: f32 = 5.387206;

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

        let d_lattitude = 0.36 * (wgs84_lattitude - REF_WGS_84_X);
        let d_longitude = 0.36 * (wgs84_longitude - REF_WGS_84_Y);

        let mut calc_latt: f32 = 0.0;
        let mut calc_long: f32 = 0.0;

        for p in 0..4 {
            for q in 0..5 {
                calc_latt += R_PQ[p][q] * d_lattitude.powi(p as i32) * d_longitude.powi(q as i32);
                calc_long += S_PQ[p][q] * d_lattitude.powi(p as i32) * d_longitude.powi(q as i32);
            }
        }

		// convert to km
        let rd_x_coordinate = (REF_RD_X + calc_latt) / 1000.0;
        let rd_y_coordinate = (REF_RD_Y + calc_long) / 1000.0;
        
        (rd_x_coordinate, rd_y_coordinate)
    }

    #[test]
    fn amsterdam_westertoren_to_rd() {
        let (x, y) : (f32, f32) = convert_to_rijksdriehoek(52.37453253, 4.88352559);
        assert!((x - 120.700723).abs() < 0.001, "{} != {}", x, 120.700723);
        assert!((y - 487.525502).abs() < 0.001, "{} != {}", y, 487.525502);
    }
    
        #[test]
    fn groningen_martinitoren_to_rd() {
        let (x, y) : (f32, f32) = convert_to_rijksdriehoek(53.21938317, 6.56820053);
        assert!((x - 233.883131).abs() < 0.001, "{} != {}", x, 233.883131);
        assert!((y - 582.065168).abs() < 0.001, "{} != {}", y, 582.065168);
    }

}
