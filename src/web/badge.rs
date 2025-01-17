//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*
use iced_web::{css, Align, Background, Bus, Css, Element, Length, Widget};
use dodrio::bumpalo;

pub use crate::style::badge::{Style, StyleSheet};

/// A badge for color highlighting small information.
/// 
/// TODO: Example
#[allow(missing_debug_implementations)]
pub struct Badge<'a, Message> {
    padding: u16,
    width: Length,
    height: Length,
    horizontal_alignment: Align,
    vertical_alignment: Align,
    style: Box<dyn StyleSheet>,
    content: Element<'a, Message>,
}

impl<'a, Message> Badge<'a, Message> {

    /// Creates a new [`Badge`](Badge) with the given content.
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message>>,
    {
        Badge {
            padding: 7,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Align::Center,
            vertical_alignment: Align::Center,
            style: Default::default(),
            content: content.into(),
        }
    }

    /// Sets the padding of the [`Badge`](Badge).
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Badge`](Badge).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Badge`](Badge).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal alignment of the content of the [`Badge`](Badge).
    pub fn align_x(mut self, alignment: Align) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the content of the [`Badge`](Badge).
    pub fn align_y(mut self, alignment: Align) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the style of the [`Badge`](Badge).
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Badge<'a, Message>
where
    Message: 'static + Clone,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        // TODO: State-based styling
        // (https://github.com/hecrj/iced/blob/master/web/src/widget/button.rs#L144)
        let style = self.style.active();

        let padding_class =
            style_sheet.insert(bump, css::Rule::Padding(self.padding));

        let border_color = match style.border_color {
            None => String::from("none"),
            Some(border_color) => css::color(border_color),
        };

        let class = {
            use dodrio::bumpalo::collections::String;

            String::from_str_in(&padding_class, bump).into_bump_str()
        };

        let node = div(bump)
            .attr("class", class)
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "background: {}; border-radius: {}rem; width:{}; height: {} \
                    border: {}px solid {}; display: inline-block; color: {}",
                    //css::color(style.background)
                    match style.background {
                        Background::Color(color) => css::color(color),
                    },
                    10, // TODO
                    css::length(self.width),
                    css::length(self.height),
                    style.border_width,
                    border_color,
                    css::color(style.text_color)
                )
                .into_bump_str(),
            )
            .children(vec![self.content.node(bump, bus, style_sheet)]);

        node.finish()
    }
}

impl<'a, Message> From<Badge<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(badge: Badge<'a, Message>) -> Element<'a, Message> {
        Element::new(badge)
    }
}