/*
use std::ffi::c_void;
use std::ops::Range;
use parley::*;
use parley::layout::Alignment;
use parley::style::{Brush, FontStack, FontStretch, FontStyle, FontWeight, StyleProperty, TextStyle};
use parley::swash::{GlyphId, NormalizedCoord, Setting, Synthesis};
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
    pub(crate) unsafe fn to_parley<'a>(&self, font_stack: &'a str) -> TextStyle<'a, [u8; 4]> {

        let default = TextStyle::default();

        TextStyle {
            font_stack: FontStack::Source(font_stack),
            font_size: self.font_size,
            font_stretch: FontStretch::from_percentage(self.font_stretch),
            font_style: match self.font_style {
                0 => FontStyle::Normal,
                1 => FontStyle::Italic,
                2 => FontStyle::Oblique(Some(4.0f32)),
                _ => unreachable!(),
            },
            font_weight: FontWeight::new(self.font_weight),
            font_variations: default.font_variations,
            font_features: default.font_features,
            locale: None,
            brush: default.brush,
            has_underline: false,
            underline_offset: None,
            underline_size: None,
            underline_brush: None,
            has_strikethrough: false,
            strikethrough_offset: None,
            strikethrough_size: None,
            strikethrough_brush: None,
            line_height: self.line_height,
            word_spacing: 0.0,
            letter_spacing: self.letter_spacing,
        }
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
pub unsafe extern "C" fn parley_new_builder() -> *mut ParleyCustomBuilder
{
    Box::into_raw(Box::new(ParleyCustomBuilder {
        entries: Vec::new(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn parley_free_builder(builder: *mut ParleyCustomBuilder)
{
    let _ = Box::from_raw(builder);
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
    parley_custom_builder: *mut ParleyCustomBuilder,
    display_scale: f32,
    max_advance: f32,
    alignment: Alignment,
    white_space_collapse: WhiteSpaceCollapse,
    run_callback: extern "C" fn(CSRunData, *mut c_void) -> (),
    context: *mut c_void,
)// -> *mut ParleyLayoutResult
{
    let layout_context = &mut *layout_context;
    let font_context = &mut *font_context;
    let raw_style = &*raw_style;

    let raw_font_family_cstr = U16CStr::from_ptr(raw_style.font_family_ptr, raw_style.font_family_len)
        .unwrap().to_string_lossy();
    let raw_font_family_str = raw_font_family_cstr.as_str();
    let raw_style = raw_style.to_parley(raw_font_family_str);

    let mut builder = layout_context.tree_builder(font_context, display_scale, &raw_style);

    let parley_custom_builder = unsafe { Box::from_raw(parley_custom_builder) };

    for entry in parley_custom_builder.entries {
        match entry {
            ParleyCustomBuilderEntry::TextStyle(style) => {
                let font_family_cstr = U16CStr::from_ptr(style.font_family_ptr, style.font_family_len)
                    .unwrap().to_string_lossy().to_owned();
                let font_family_str = &*font_family_cstr;
                let style = style.to_parley(font_family_str);
                builder.push_style_span(style);
            }
            ParleyCustomBuilderEntry::Text(text) => {
                let text = text.to_string_lossy();
                println!("text: {}", text.as_str());
                builder.push_text(text.as_str());
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

    let (mut layout, _text) = builder.build();

    let max_advance = if max_advance == 0.0 {
        None
    } else {
        Some(max_advance)
    };

    layout.break_all_lines(max_advance);

    for line in layout.lines() {
        // Iterate over GlyphRun's within each line
        let mut run_index = 0u16;

        println!("line: {:?}", line.metrics());

        for glyph_run in line.runs() {

            let syn = glyph_run.synthesis();
            let syn_vars = syn.variations();

            let font = glyph_run.font();
            let font_index = font.index as usize;

            let mut clusters: Vec<CSClusterData> = Vec::new();
            let mut glyphs: Vec<CSGlyphData> = Vec::new();

            let mut cluster_index = 0u16;
            for cluster in glyph_run.clusters() {
                for glyph in cluster.glyphs() {
                    let cs_glyph_data = CSGlyphData {
                        run_index: run_index as u32,
                        cluster_index: cluster_index as u32,
                        x: glyph.x,
                        y: glyph.y,
                        advance: glyph.advance,
                        style_index: glyph.style_index as u32,
                        glyph_id: glyph.id,
                    };
                    glyphs.push(cs_glyph_data);
                    println!("x: {} y: {}", glyph.x, glyph.y);
                }

                let cs_cluster_data = CSClusterData {
                    run_index: run_index,
                };
                clusters.push(cs_cluster_data);
                cluster_index += 1;
            }

            let glyph_count = glyphs.len();
            let glyph_byte_buffer = ByteBuffer::from_vec_struct(glyphs);

            let cs_run_data = CSRunData {
                font_index,
                font_size: glyph_run.font_size(),

                text_range_start: glyph_run.text_range().start as u32,
                text_range_end: glyph_run.text_range().end as u32,
                baseline: line.metrics().baseline,
                offset: line.metrics().offset,
                trailing_whitespace: line.metrics().trailing_whitespace,

                glyph_start: 0,
                glyph_byte_buffer,
                glyph_count,

                word_spacing: 0f32,
                letter_spacing: 0f32,
                advance: glyph_run.advance(),
            };

            run_callback(cs_run_data, context);

            run_index += 1;
        }
    }

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

/// Metrics information for a run.
#[repr(C)]
pub struct CSRunMetrics {
    /// Typographic ascent.
    pub ascent: f32,
    /// Typographic descent.
    pub descent: f32,
    /// Typographic leading.
    pub leading: f32,
    /// Offset of the top of underline decoration from the baseline.
    pub underline_offset: f32,
    /// Thickness of the underline decoration.
    pub underline_size: f32,
    /// Offset of the top of strikethrough decoration from the baseline.
    pub strikethrough_offset: f32,
    /// Thickness of the strikethrough decoration.
    pub strikethrough_size: f32,
}

#[repr(C)]
pub struct CSSynthesis {
    vars: [Setting<f32>; 4],
    len: u8,
    embolden: bool,
    skew: i8,
}


#[repr(C)]
pub struct CSClusterData {
    pub run_index: u16,
}

#[repr(C)]
pub struct CSGlyphData {
    pub x: f32,
    pub y: f32,
    pub advance: f32,
    pub style_index: u32,
    pub run_index: u32,
    pub cluster_index: u32,
    pub glyph_id: u16,
}

#[repr(C)]
pub struct CSRunData {
    pub glyph_byte_buffer: ByteBuffer,
    pub glyph_count: usize,
    /// Index of the font for the run.
    pub font_index: usize,
    /// Font size.
    pub font_size: f32,
    /// Start of the source text.
    pub text_range_start: u32,
    /// End of the source text.
    pub text_range_end: u32,
    pub offset : f32,
    pub baseline: f32,
    pub trailing_whitespace : f32,
    /// Base for glyph indices.
    pub glyph_start: usize,
    /// Additional word spacing.
    pub word_spacing: f32,
    /// Additional letter spacing.
    pub letter_spacing: f32,
    /// Total advance of the run.
    pub advance: f32,
}
*/