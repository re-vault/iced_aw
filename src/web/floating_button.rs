//! Use a floating button to overlay a button over some content
//! 
//! *This API requires the following crate features to be activated: floating_button*
use iced_web::{Bus, Button, Css, Element, Widget};
use dodrio::bumpalo;

pub use crate::style::button::*;

pub mod anchor;
pub use anchor::Anchor;

pub mod offset;
pub use offset::Offset;

/// A floating button floating over some content.
/// 
/// TODO: Example
#[allow(missing_debug_implementations)]
pub struct FloatingButton<'a, Message> {
    anchor: Anchor,
    offset: Offset,
    hidden: bool,
    on_press: Option<Message>,
    underlay: Element<'a, Message>,
    button: Button<'a, Message>,
}

impl<'a, Message> FloatingButton<'a, Message>
where 
    Message: Clone,
{

    /// Creates a new [`FloatingButton`](FloatingButton) over some content,
    /// showing the given [`Button`](iced_native::button::Button).
    pub fn new<U, B>(underlay: U, button: B) -> Self
    where
        U: Into<Element<'a, Message>>,
        B: Into<Button<'a, Message>>,
    {
        FloatingButton {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            on_press: None,
            underlay: underlay.into(),
            button: button.into(),
        }
    }

    /// Sets the [`Anchor`](Anchor) of the [`FloatingButton`](FloatingButton).
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the [`Offset`](Offset) of the [`FloatingButton`](FloatingButton).
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Hide or unhide the [`Button`](iced_native::button::Button) on the
    /// [`FloatingButton`](FloatingButton).
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// Sets the `on_press` message for the [`Button`].
    /// 
    /// This is currently only a workaround.
   pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg.clone());
        self.button = self.button.on_press(msg);
        self
    }
}

impl<'a, Message> Widget<Message> for FloatingButton<'a, Message>
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

        let position = match self.anchor {
            Anchor::NorthWest => format!("top: {}px; left: {}px;", self.offset.y, self.offset.x),
            Anchor::NorthEast => format!("top: {}px; right: {}px;", self.offset.y, self.offset.x),
            Anchor::SouthWest => format!("bottom: {}px; left: {}px;", self.offset.y, self.offset.x),
            Anchor::SouthEast => format!("bottom: {}px; right: {}px;", self.offset.y, self.offset.x),
        };

        let node = div(bump)
            .attr("style", "position: relative; width: 100%; height: 100%;")
            .children(vec![
                self.underlay.node(bump, bus, style_sheet),
                div(bump)
                    .attr(
                        "style",
                        bumpalo::format!(
                            in bump,
                            "position: absolute; {}",
                            position
                        ).into_bump_str(),
                    )
                    .children(vec![self.button.node(bump, bus, style_sheet)])
                    .finish(),
            ]);

        node.finish()
    }
}

impl<'a, Message> From<FloatingButton<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(floating_button: FloatingButton<'a, Message>) -> Element<'a, Message> {
        Element::new(floating_button)
    }
}