//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;

pub(crate) mod no_confusing_arrow;
pub(crate) mod no_control_characters_in_regex;
pub(crate) mod no_duplicate_jsx_props;
pub(crate) mod no_excessive_complexity;
pub(crate) mod no_fallthrough_switch_clause;
pub(crate) mod no_for_each;
pub(crate) mod no_nonoctal_decimal_escape;
pub(crate) mod no_self_assign;
pub(crate) mod no_static_only_class;
pub(crate) mod no_useless_empty_export;
pub(crate) mod no_void;
pub(crate) mod use_arrow_function;
pub(crate) mod use_grouped_type_import;
pub(crate) mod use_heading_content;
pub(crate) mod use_import_restrictions;
pub(crate) mod use_literal_enum_members;
pub(crate) mod use_literal_keys;
pub(crate) mod use_simple_number_keys;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_confusing_arrow :: NoConfusingArrow ,
            self :: no_control_characters_in_regex :: NoControlCharactersInRegex ,
            self :: no_duplicate_jsx_props :: NoDuplicateJsxProps ,
            self :: no_excessive_complexity :: NoExcessiveComplexity ,
            self :: no_fallthrough_switch_clause :: NoFallthroughSwitchClause ,
            self :: no_for_each :: NoForEach ,
            self :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscape ,
            self :: no_self_assign :: NoSelfAssign ,
            self :: no_static_only_class :: NoStaticOnlyClass ,
            self :: no_useless_empty_export :: NoUselessEmptyExport ,
            self :: no_void :: NoVoid ,
            self :: use_arrow_function :: UseArrowFunction ,
            self :: use_grouped_type_import :: UseGroupedTypeImport ,
            self :: use_heading_content :: UseHeadingContent ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_literal_enum_members :: UseLiteralEnumMembers ,
            self :: use_literal_keys :: UseLiteralKeys ,
            self :: use_simple_number_keys :: UseSimpleNumberKeys ,
        ]
     }
}
