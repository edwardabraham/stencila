use codec::CodecTrait;
use codec_html::HtmlCodec;
use test_props::{node, proptest::prelude::*, Freedom};
use test_utils::assert_json_eq;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    fn test(input in node(Freedom::Max)) {
        let string = HtmlCodec::to_string(&input, None).unwrap();
        let output = HtmlCodec::from_str(&string, None).unwrap();
        assert_json_eq!(input, output)
    }
}