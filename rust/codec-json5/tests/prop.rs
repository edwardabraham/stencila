use codec::CodecTrait;
use codec_json5::Json5Codec;
use test_props::{node, proptest::prelude::*, Freedom};
use test_utils::assert_json_eq;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    // JSON5 does not appear to deal with Unicode characters (?)
    // so this test uses `Low` freedom for now
    #[test]
    fn test(input in node(Freedom::Low)) {
        let string = Json5Codec::to_string(&input, None).unwrap();
        let output = Json5Codec::from_str(&string, None).unwrap();
        assert_json_eq!(input, output)
    }
}
