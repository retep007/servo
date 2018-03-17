/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />
<% from data import Keyword %>
<% data.new_style_struct("InheritedText", inherited=True, gecko_name="Text") %>

${helpers.predefined_type("line-height",
                          "LineHeight",
                          "computed::LineHeight::normal()",
                          animation_value_type="LineHeight",
                          flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
                          spec="https://drafts.csswg.org/css2/visudet.html#propdef-line-height",
                          servo_restyle_damage = "reflow")}

// CSS Text Module Level 3

// TODO(pcwalton): `full-width`
${helpers.single_keyword("text-transform",
                         "none capitalize uppercase lowercase",
                         extra_gecko_values="full-width",
                         animation_value_type="discrete",
                         flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
                         spec="https://drafts.csswg.org/css-text/#propdef-text-transform",
                         servo_restyle_damage="rebuild_and_reflow")}

${helpers.single_keyword("hyphens", "manual none auto",
                         gecko_enum_prefix="StyleHyphens",
                         products="gecko", animation_value_type="discrete", extra_prefixes="moz",
                         spec="https://drafts.csswg.org/css-text/#propdef-hyphens")}

// TODO: Support <percentage>
${helpers.single_keyword("-moz-text-size-adjust", "auto none",
                         gecko_constant_prefix="NS_STYLE_TEXT_SIZE_ADJUST",
                         gecko_ffi_name="mTextSizeAdjust",
                         products="gecko", animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-size-adjust/#adjustment-control",
                         alias="-webkit-text-size-adjust")}

${helpers.predefined_type("text-indent",
                          "LengthOrPercentage",
                          "computed::LengthOrPercentage::Length(computed::Length::new(0.))",
                          animation_value_type="ComputedValue",
                          spec="https://drafts.csswg.org/css-text/#propdef-text-indent",
                          allow_quirks=True, servo_restyle_damage = "reflow")}

// Also known as "word-wrap" (which is more popular because of IE), but this is the preferred
// name per CSS-TEXT 6.2.
${helpers.single_keyword("overflow-wrap",
                         "normal break-word",
                         gecko_constant_prefix="NS_STYLE_OVERFLOWWRAP",
                         animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-text/#propdef-overflow-wrap",
                         alias="word-wrap",
                         servo_restyle_damage="rebuild_and_reflow")}

// TODO(pcwalton): Support `word-break: keep-all` once we have better CJK support.
${helpers.single_keyword("word-break",
                         "normal break-all keep-all",
                         gecko_constant_prefix="NS_STYLE_WORDBREAK",
                         animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-text/#propdef-word-break",
                         servo_restyle_damage="rebuild_and_reflow")}

// TODO(pcwalton): Support `text-justify: distribute`.
<%helpers:single_keyword
    name="text-justify"
    values="auto none inter-word"
    extra_gecko_values="inter-character"
    extra_specified="${'distribute' if product == 'gecko' else ''}"
    gecko_enum_prefix="StyleTextJustify"
    animation_value_type="discrete"
    gecko_pref="layout.css.text-justify.enabled"
    flags="APPLIES_TO_PLACEHOLDER",
    spec="https://drafts.csswg.org/css-text/#propdef-text-justify"
    servo_restyle_damage="rebuild_and_reflow"
>
    % if product == 'gecko':
    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value(&self, _: &Context) -> computed_value::T {
            match *self {
                % for value in "Auto None InterCharacter InterWord".split():
                SpecifiedValue::${value} => computed_value::T::${value},
                % endfor
                // https://drafts.csswg.org/css-text-3/#valdef-text-justify-distribute
                SpecifiedValue::Distribute => computed_value::T::InterCharacter,
            }
        }

        #[inline]
        fn from_computed_value(computed: &computed_value::T) -> SpecifiedValue {
            match *computed {
                % for value in "Auto None InterCharacter InterWord".split():
                computed_value::T::${value} => SpecifiedValue::${value},
                % endfor
            }
        }
    }
    % endif
</%helpers:single_keyword>

${helpers.single_keyword("text-align-last",
                         "auto start end left right center justify",
                         products="gecko",
                         gecko_constant_prefix="NS_STYLE_TEXT_ALIGN",
                         animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-text/#propdef-text-align-last")}

// TODO make this a shorthand and implement text-align-last/text-align-all
//
// FIXME(emilio): This can't really be that complicated.
${helpers.predefined_type("text-align",
                          "TextAlign",
                          "computed::TextAlign::start()",
                          animation_value_type="discrete",
                          flags="APPLIES_TO_PLACEHOLDER",
                          spec="https://drafts.csswg.org/css-text/#propdef-text-align",
                          servo_restyle_damage = "reflow")}

${helpers.predefined_type("letter-spacing",
                          "LetterSpacing",
                          "computed::LetterSpacing::normal()",
                          animation_value_type="ComputedValue",
                          flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
                          spec="https://drafts.csswg.org/css-text/#propdef-letter-spacing",
                          servo_restyle_damage="rebuild_and_reflow")}

${helpers.predefined_type("word-spacing",
                          "WordSpacing",
                          "computed::WordSpacing::normal()",
                          animation_value_type="ComputedValue",
                          flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
                          spec="https://drafts.csswg.org/css-text/#propdef-word-spacing",
                          servo_restyle_damage="rebuild_and_reflow")}

<%helpers:single_keyword
    name="white-space"
    values="normal pre nowrap pre-wrap pre-line"
    extra_gecko_values="-moz-pre-space"
    gecko_enum_prefix="StyleWhiteSpace"
    needs_conversion="True"
    animation_value_type="discrete"
    // Only allowed for UA sheets, which set it !important.
    flags="APPLIES_TO_PLACEHOLDER"
    spec="https://drafts.csswg.org/css-text/#propdef-white-space"
    servo_restyle_damage="rebuild_and_reflow"
>
    % if product != "gecko":
    impl SpecifiedValue {
        pub fn allow_wrap(&self) -> bool {
            match *self {
                SpecifiedValue::Nowrap |
                SpecifiedValue::Pre => false,
                SpecifiedValue::Normal |
                SpecifiedValue::PreWrap |
                SpecifiedValue::PreLine => true,
            }
        }

        pub fn preserve_newlines(&self) -> bool {
            match *self {
                SpecifiedValue::Normal |
                SpecifiedValue::Nowrap => false,
                SpecifiedValue::Pre |
                SpecifiedValue::PreWrap |
                SpecifiedValue::PreLine => true,
            }
        }

        pub fn preserve_spaces(&self) -> bool {
            match *self {
                SpecifiedValue::Normal |
                SpecifiedValue::Nowrap |
                SpecifiedValue::PreLine => false,
                SpecifiedValue::Pre |
                SpecifiedValue::PreWrap => true,
            }
        }
    }
    % endif
</%helpers:single_keyword>

${helpers.predefined_type(
    "text-shadow",
    "SimpleShadow",
    None,
    vector=True,
    animation_value_type="AnimatedTextShadowList",
    ignored_when_colors_disabled=True,
    flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
    spec="https://drafts.csswg.org/css-text-decor-3/#text-shadow-property",
)}

${helpers.predefined_type(
    "text-emphasis-style",
    "TextEmphasisStyle",
    None,
    initial_specified_value="SpecifiedValue::None",
    products="gecko",
    boxed=True,
    animation_value_type="discrete",
    spec="https://drafts.csswg.org/css-text-decor/#propdef-text-emphasis-style",
)}

<%helpers:longhand name="text-emphasis-position" animation_value_type="discrete" products="gecko"
                   spec="https://drafts.csswg.org/css-text-decor/#propdef-text-emphasis-position">
    #[derive(Clone, Copy, Debug, Eq, MallocSizeOf, Parse, PartialEq)]
    #[derive(ToComputedValue, ToCss)]
    pub enum HorizontalWritingModeValue {
        Over,
        Under,
    }

    #[derive(Clone, Copy, Debug, Eq, MallocSizeOf, Parse, PartialEq)]
    #[derive(ToComputedValue, ToCss)]
    pub enum VerticalWritingModeValue {
        Right,
        Left,
    }

    #[derive(Clone, Debug, MallocSizeOf, PartialEq, ToComputedValue, ToCss)]
    pub struct SpecifiedValue(pub HorizontalWritingModeValue, pub VerticalWritingModeValue);

    pub mod computed_value {
        pub type T = super::SpecifiedValue;
    }

    pub fn get_initial_value() -> computed_value::T {
        SpecifiedValue(HorizontalWritingModeValue::Over, VerticalWritingModeValue::Right)
    }

    pub fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>)
                         -> Result<SpecifiedValue, ParseError<'i>> {
       if let Ok(horizontal) = input.try(|input| HorizontalWritingModeValue::parse(input)) {
            let vertical = VerticalWritingModeValue::parse(input)?;
            Ok(SpecifiedValue(horizontal, vertical))
        } else {
            let vertical = VerticalWritingModeValue::parse(input)?;
            let horizontal = HorizontalWritingModeValue::parse(input)?;
            Ok(SpecifiedValue(horizontal, vertical))
        }
    }

    % if product == "gecko":
        impl SpecifiedValue {
            pub fn from_gecko_keyword(kw: u32) -> Self {
                use gecko_bindings::structs;

                let vert = if kw & structs::NS_STYLE_TEXT_EMPHASIS_POSITION_RIGHT != 0 {
                    VerticalWritingModeValue::Right
                } else {
                    debug_assert!(kw & structs::NS_STYLE_TEXT_EMPHASIS_POSITION_LEFT != 0);
                    VerticalWritingModeValue::Left
                };
                let horiz = if kw & structs::NS_STYLE_TEXT_EMPHASIS_POSITION_OVER != 0 {
                    HorizontalWritingModeValue::Over
                } else {
                    debug_assert!(kw & structs::NS_STYLE_TEXT_EMPHASIS_POSITION_UNDER != 0);
                    HorizontalWritingModeValue::Under
                };
                SpecifiedValue(horiz, vert)
            }
        }

        impl From<u8> for SpecifiedValue {
            fn from(bits: u8) -> SpecifiedValue {
                SpecifiedValue::from_gecko_keyword(bits as u32)
            }
        }

        impl From<SpecifiedValue> for u8 {
            fn from(v: SpecifiedValue) -> u8 {
                use gecko_bindings::structs;

                let mut result = match v.0 {
                    HorizontalWritingModeValue::Over => structs::NS_STYLE_TEXT_EMPHASIS_POSITION_OVER,
                    HorizontalWritingModeValue::Under => structs::NS_STYLE_TEXT_EMPHASIS_POSITION_UNDER,
                };
                match v.1 {
                    VerticalWritingModeValue::Right => {
                        result |= structs::NS_STYLE_TEXT_EMPHASIS_POSITION_RIGHT;
                    }
                    VerticalWritingModeValue::Left => {
                        result |= structs::NS_STYLE_TEXT_EMPHASIS_POSITION_LEFT;
                    }
                };
                result as u8
            }
        }
    % endif
</%helpers:longhand>

${helpers.predefined_type(
    "text-emphasis-color",
    "Color",
    "computed_value::T::currentcolor()",
    initial_specified_value="specified::Color::currentcolor()",
    products="gecko",
    animation_value_type="AnimatedColor",
    ignored_when_colors_disabled=True,
    spec="https://drafts.csswg.org/css-text-decor/#propdef-text-emphasis-color",
)}

${helpers.predefined_type(
    "-moz-tab-size",
    "MozTabSize",
    "generics::text::MozTabSize::Number(From::from(8.0))",
    products="gecko",
    animation_value_type="AnimatedMozTabSize",
    spec="https://drafts.csswg.org/css-text-3/#tab-size-property",
)}

// CSS Compatibility
// https://compat.spec.whatwg.org
${helpers.predefined_type(
    "-webkit-text-fill-color",
    "Color",
    "computed_value::T::currentcolor()",
    products="gecko",
    gecko_pref="layout.css.prefixes.webkit",
    animation_value_type="AnimatedColor",
    ignored_when_colors_disabled=True,
    flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
    spec="https://compat.spec.whatwg.org/#the-webkit-text-fill-color",
)}

${helpers.predefined_type(
    "-webkit-text-stroke-color",
    "Color",
    "computed_value::T::currentcolor()",
    initial_specified_value="specified::Color::currentcolor()",
    products="gecko",
    animation_value_type="AnimatedColor",
    ignored_when_colors_disabled=True,
    gecko_pref="layout.css.prefixes.webkit",
    flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
    spec="https://compat.spec.whatwg.org/#the-webkit-text-stroke-color",
)}

${helpers.predefined_type("-webkit-text-stroke-width",
                          "BorderSideWidth",
                          "::values::computed::NonNegativeLength::new(0.)",
                          initial_specified_value="specified::BorderSideWidth::Length(specified::Length::zero())",
                          computed_type="::values::computed::NonNegativeLength",
                          products="gecko",
                          gecko_pref="layout.css.prefixes.webkit",
                          flags="APPLIES_TO_FIRST_LETTER APPLIES_TO_FIRST_LINE APPLIES_TO_PLACEHOLDER",
                          spec="https://compat.spec.whatwg.org/#the-webkit-text-stroke-width",
                          animation_value_type="discrete")}

// CSS Ruby Layout Module Level 1
// https://drafts.csswg.org/css-ruby/
${helpers.single_keyword("ruby-align", "space-around start center space-between",
                         products="gecko", animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-ruby/#ruby-align-property")}

${helpers.single_keyword("ruby-position", "over under",
                         products="gecko", animation_value_type="discrete",
                         spec="https://drafts.csswg.org/css-ruby/#ruby-position-property")}

// CSS Writing Modes Module Level 3
// https://drafts.csswg.org/css-writing-modes-3/

${helpers.single_keyword("text-combine-upright", "none all",
                         products="gecko", animation_value_type="discrete",
                         gecko_pref="layout.css.text-combine-upright.enabled",
                         spec="https://drafts.csswg.org/css-writing-modes-3/#text-combine-upright")}

// SVG 1.1: Section 11 - Painting: Filling, Stroking and Marker Symbols
${helpers.single_keyword("text-rendering",
                         "auto optimizespeed optimizelegibility geometricprecision",
                         animation_value_type="discrete",
                         spec="https://www.w3.org/TR/SVG11/painting.html#TextRenderingProperty",
                         servo_restyle_damage="rebuild_and_reflow")}

// FIXME Firefox expects the initial value of this property to change depending
// on the value of the layout.css.control-characters.visible pref.
${helpers.single_keyword("-moz-control-character-visibility",
                         "hidden visible",
                         gecko_constant_prefix="NS_STYLE_CONTROL_CHARACTER_VISIBILITY",
                         gecko_ffi_name="mControlCharacterVisibility",
                         animation_value_type="none",
                         products="gecko",
                         spec="Nonstandard")}
