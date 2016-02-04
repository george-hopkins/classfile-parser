use nom::{
  be_u16,
  IResult,
};

use attribute_info::attribute_parser;

use method_info::MethodInfo;

pub fn method_parser(input: &[u8]) -> IResult<&[u8], MethodInfo> {
    chain!(input,
        access_flags: be_u16 ~
        name_index: be_u16 ~
        descriptor_index: be_u16 ~
        attributes_count: be_u16 ~
        attributes: count!(attribute_parser, attributes_count as usize),
        || {
            MethodInfo {
                access_flags: access_flags,
                name_index: name_index,
                descriptor_index: descriptor_index,
                attributes_count: attributes_count,
                attributes: attributes,
            }
        }
    )
}