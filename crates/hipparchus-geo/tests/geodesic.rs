use float_cmp::assert_approx_eq;
use hipparchus_geo::geodesic::Geodesic;
use hipparchus_geo::geodesic::DirectGeodesic;
use hipparchus_geo::geodesic::InverseGeodesic;
use hipparchus_geo::earth::models::WGS84;
use std::io::BufRead;

// *_geodtest_* tests are based on Karney's GeodTest*.dat test datasets.
// A description of these files' content can be found at:
//     https://geographiclib.sourceforge.io/html/geodesic.html#testgeod
// Here are some key excerpts...
//    This consists of a set of geodesics for the WGS84 ellipsoid.
//     Each line of the test set gives 10 space delimited numbers
//         latitude at point 1, lat1 (degrees, exact)
//         longitude at point 1, lon1 (degrees, always 0)
//         azimuth at point 1, azi1 (clockwise from north in degrees, exact)
//         latitude at point 2, lat2 (degrees, accurate to 10−18 deg)
//         longitude at point 2, lon2 (degrees, accurate to 10−18 deg)
//         azimuth at point 2, azi2 (degrees, accurate to 10−18 deg)
//         geodesic distance from point 1 to point 2, s12 (meters, exact)
//         arc distance on the auxiliary sphere, a12 (degrees, accurate to 10−18 deg)
//         reduced length of the geodesic, m12 (meters, accurate to 0.1 pm)
//         the area under the geodesic, S12 (m2, accurate to 1 mm2)

static FULL_TEST_PATH: &str = "test_fixtures/test_data_unzipped/GeodTest.dat";
static SHORT_TEST_PATH: &str = "test_fixtures/test_data_unzipped/GeodTest-short.dat";
static BUILTIN_TEST_PATH: &str = "tests/geotest-100.dat";
fn test_input_path() -> &'static str 
{
    if cfg!(feature = "test_full") 
    {
        FULL_TEST_PATH
    } else if cfg!(feature = "test_short") 
    {
        SHORT_TEST_PATH
    } 
    else
    {
        BUILTIN_TEST_PATH
    }
}

fn geodtest_basic<T>(path: &str, f: T)
where
    T: Fn(usize, &(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)),
{
    let dir_base = std::env::current_dir().expect("Failed to determine current directory");
    let path_base = dir_base.as_path();
    let pathbuf = std::path::Path::new(path_base).join(path);
    let path = pathbuf.as_path();
    let file = match std::fs::File::open(path) 
    {
        Ok(val) => val,
        Err(_error) => 
        {
            let path_str = path
                .to_str()
                .expect("Failed to convert GeodTest path to string during error reporting");
            panic!("Failed to open test input file. Run `script/download-test-data.sh` to download test input to: {}\nFor details see https://geographiclib.sourceforge.io/html/geodesic.html#testgeod", path_str)
        }
    };
    let reader = std::io::BufReader::new(file);
    reader.lines().enumerate().for_each(|(i, line)| 
    {
        let line_safe = line.expect("Failed to read line");
        let items: Vec<f64> = line_safe
            .split(' ')
            .enumerate()
            .map(|(j, item)| match item.parse::<f64>() {
                Ok(parsed) => parsed,
                Err(_error) => {
                    panic!("Error parsing item {} on line {}: {}", j + 1, i + 1, item)
                }
            })
            .collect();
        assert_eq!(items.len(), 10);
        let tuple = 
        (
            items[0], items[1], items[2], items[3], items[4], items[5], items[6], items[7],
            items[8], items[9],
        );
        f(i + 1, &tuple); // report 1-based line number rather than 0-based
    });
}

#[test]
fn test_geodtest_geodesic_direct12() 
{
    let g = std::sync::Arc::new(std::sync::Mutex::new(Geodesic::model::<WGS84>()));

    geodtest_basic(
        test_input_path(),
        |_line_num, &(lat1, lon1, azi1, lat2, lon2, azi2, s12, a12, m12, S12)| {
            let g = g.lock().unwrap();
            let (lat2_out, lon2_out, azi2_out, m12_out, _M12_out, _M21_out, S12_out, a12_out) =
                g.direct(lat1, lon1, azi1, s12);
            assert_approx_eq!(f64, lat2, lat2_out, epsilon = 1e-13);
            assert_approx_eq!(f64, lon2, lon2_out, epsilon = 2e-8);
            assert_approx_eq!(f64, azi2, azi2_out, epsilon = 2e-8);
            assert_approx_eq!(f64, m12, m12_out, epsilon = 9e-9);
            assert_approx_eq!(f64, S12, S12_out, epsilon = 2e4); // Note: unreasonable tolerance
            assert_approx_eq!(f64, a12, a12_out, epsilon = 9e-14);
        },
    );
}

#[test]
fn test_geodtest_geodesic_direct21() 
{
    let g = std::sync::Arc::new(std::sync::Mutex::new(Geodesic::model::<WGS84>()));

    geodtest_basic(
        test_input_path(),
        |_line_num, &(lat1, lon1, azi1, lat2, lon2, azi2, s12, a12, m12, S12)| {
            let g = g.lock().unwrap();
            // Reverse some values for 2->1 instead of 1->2
            let (lat1, lon1, azi1, lat2, lon2, azi2, s12, a12, m12, S12) =
                (lat2, lon2, azi2, lat1, lon1, azi1, -s12, -a12, -m12, -S12);
            let (lat2_out, lon2_out, azi2_out, m12_out, _M12_out, _M21_out, S12_out, a12_out) =
                g.direct(lat1, lon1, azi1, s12);
            assert_approx_eq!(f64, lat2, lat2_out, epsilon = 8e-14);
            assert_approx_eq!(f64, lon2, lon2_out, epsilon = 4e-6);
            assert_approx_eq!(f64, azi2, azi2_out, epsilon = 4e-6);
            assert_approx_eq!(f64, m12, m12_out, epsilon = 1e-8);
            assert_approx_eq!(f64, S12, S12_out, epsilon = 3e6); // Note: unreasonable tolerance
            assert_approx_eq!(f64, a12, a12_out, epsilon = 9e-14);
        },
    );
}

#[test]
fn test_geodtest_geodesic_inverse12() 
{
    let g = std::sync::Arc::new(std::sync::Mutex::new(Geodesic::model::<WGS84>()));

    geodtest_basic(
        test_input_path(),
        |line_num, &(lat1, lon1, azi1, lat2, lon2, azi2, s12, a12, m12, S12)| {
            let g = g.lock().unwrap();
            let (s12_out, azi1_out, azi2_out, m12_out, _M12_out, _M21_out, S12_out, a12_out) =
                g.inverse(lat1, lon1, lat2, lon2);
            assert_approx_eq!(f64, s12, s12_out, epsilon = 8e-9);
            assert_approx_eq!(f64, azi1, azi1_out, epsilon = 2e-2);
            assert_approx_eq!(f64, azi2, azi2_out, epsilon = 2e-2);
            assert_approx_eq!(f64, m12, m12_out, epsilon = 5e-5);
            // Our area calculation differs significantly (~1e7) from the value in GeodTest.dat for
            // line 400001, BUT our value also perfectly matches the value returned by GeographicLib
            // (C++) 1.51. Here's the problem line, for reference:
            // 4.199535552987 0 90 -4.199535552987 179.398106343454992238 90 19970505.608097404994 180 0
            if line_num != 400001 {
                assert_approx_eq!(f64, S12, S12_out, epsilon = 3e10); // Note: unreasonable tolerance
            }
            assert_approx_eq!(f64, a12, a12_out, epsilon = 2e-10);
        },
    );
}
