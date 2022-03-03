use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsDeclareModifier;
use rslint_parser::ast::TsDeclareModifierFields;

impl ToFormatElement for TsDeclareModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDeclareModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}