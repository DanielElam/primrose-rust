use parley::*;
use parley::style::{Brush, StyleProperty, TextStyle};
use parley::swash::scale::image::Content::Color;
use widestring::{U16CStr, U16CString};
use crate::bytebuffer::ByteBuffer;


#[repr(C)]
pub struct CustomTextStyle {
    pub font_family_ptr: *const u16,
    pub font_family_len: usize,
    pub font_size: f32,
    pub font_stretch: f32,
    pub font_style: i32,
    pub font_weight: f32,
    pub line_height: f32,
    pub word_spacing: f32,
    pub letter_spacing: f32,
}

impl CustomTextStyle {
    pub(crate) fn to_parley(&self) -> TextStyle<[u8; 4]> {
        todo!()
    }
}

pub enum ParleyCustomBuilderEntry {
    TextStyle(CustomTextStyle),
    Text(U16CString),
    InlineBox(InlineBox),
    PopStyleSpan,
}

pub struct ParleyCustomBuilder {
    pub entries: Vec<ParleyCustomBuilderEntry>,
}

#[no_mangle]
pub unsafe extern "C" fn parley_new_font_context() -> *mut FontContext
{
    Box::into_raw(Box::new(FontContext::default()))
}

#[no_mangle]
pub unsafe extern "C" fn parley_free_font_context(font_context: *mut FontContext)
{
    let _ = Box::from_raw(font_context);
}

#[no_mangle]
pub unsafe extern "C" fn parley_new_layout_context() -> *mut LayoutContext
{
    Box::into_raw(Box::new(LayoutContext::new()))
}

#[repr(C)]
pub enum WhiteSpaceCollapse {
    Collapse,
    Preserve,
}

#[no_mangle]
pub unsafe extern "C" fn parley_execute_layout(
    layout_context: *mut LayoutContext,
    font_context: *mut FontContext,
    raw_style: *const CustomTextStyle,
    parley_custom_builder: ParleyCustomBuilder,
    display_scale: f32,
    white_space_collapse: WhiteSpaceCollapse
)// -> *mut ParleyLayoutResult
{
    let layout_context = &mut *layout_context;
    let font_context = &mut *font_context;
    let raw_style = &*raw_style;

    let raw_style = raw_style.to_parley();
    let mut builder = layout_context.tree_builder(font_context, display_scale, &raw_style);

    for entry in parley_custom_builder.entries {
        match entry {
            ParleyCustomBuilderEntry::TextStyle(style) => {
                let style = style.to_parley();
                builder.push_style_span(style);
            }
            ParleyCustomBuilderEntry::Text(text) => {
                builder.push_text(text.to_string_lossy().as_str());
            }
            ParleyCustomBuilderEntry::InlineBox(inline_box) => {
                builder.push_inline_box(parley::InlineBox {
                    id: inline_box.id,
                    index: inline_box.index,
                    width: inline_box.width,
                    height: inline_box.height,
                });
            }
            ParleyCustomBuilderEntry::PopStyleSpan => {
                builder.pop_style_span();
            }
        }
    }

    builder.set_white_space_mode(match white_space_collapse  {
        WhiteSpaceCollapse::Collapse => parley::style::WhiteSpaceCollapse::Collapse,
        WhiteSpaceCollapse::Preserve => parley::style::WhiteSpaceCollapse::Preserve,
    });

    let (layout, _text) = builder.build();

    //let text = widestring::ucstr::U16CStr::from_ptr(text, len).unwrap().to_string_lossy();
    //let builder = layout_context.ranged_builder(font_context, &text, display_scale);
    //Box::into_raw(Box::new(builder))
}

#[no_mangle]
pub unsafe extern "C" fn parley_push_text_style(
    builder: *mut ParleyCustomBuilder,
    style: CustomTextStyle
)
{
    unsafe {
        let builder = &mut *builder;
        builder.entries.push(ParleyCustomBuilderEntry::TextStyle(style));
    }
}

#[no_mangle]
pub unsafe extern "C" fn parley_pop_style_span(builder: *mut ParleyCustomBuilder)
{
    unsafe {
        let builder = &mut *builder;
        builder.entries.push(ParleyCustomBuilderEntry::PopStyleSpan);
    }
}

/// Pushes a text entry onto the custom builder. **Text is copied** from the provided pointer and length.
#[no_mangle]
pub unsafe extern "C" fn parley_push_text(
    builder: *mut ParleyCustomBuilder,
    text: *const u16,
    len: usize
)
{
    unsafe {
        let builder = &mut *builder;
        builder.entries.push(ParleyCustomBuilderEntry::Text(U16CString::from_ptr(text, len).unwrap()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn parley_push_inline_box(builder: *mut ParleyCustomBuilder, inline_box: InlineBox)
{
    unsafe {
        let builder = &mut *builder;
        builder.entries.push(ParleyCustomBuilderEntry::InlineBox(inline_box));
    }
}

#[repr(C)]
pub struct InlineBox {
    /// User-specified identifier for the box, which can be used by the user to determine which box in
    /// parley's output corresponds to which box in it's input.
    pub id: u64,
    /// The character index into the underlying text string at which the box should be placed.
    pub index: usize,
    /// The width of the box in pixels
    pub width: f32,
    /// The height of the box in pixels
    pub height: f32,
}

