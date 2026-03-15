use wkt_crs_rs::{WktCrsTypes, base_types::Remark, parse_wkt_crs};

const EXAMPLE1: &str = r#"REMARK["A remark in ASCII"]"#;
const EXAMPLE2: &str = r#"REMARK["Замечание на русском языке"]"#;

#[test]
fn test_remark() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct =
		vec![WktCrsTypes::Remark(Remark("A remark in ASCII".to_string()))];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}

fn test_example_2() {
	let correct = vec![WktCrsTypes::Remark(Remark(
		"Замечание на русском языке".to_string(),
	))];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(correct, ast);
}
