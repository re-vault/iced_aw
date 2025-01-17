//! A modal for showing elements as an overlay on top of another.
//! 
//! *This API requires the following crate features to be activated: modal*
use std::hash::Hash;

use iced_native::{Clipboard, Element, Event, Layout, Point, Widget, event, overlay};

use super::overlay::modal::{self, ModalOverlay};
pub use super::overlay::modal::Renderer;

/// A modal content as an overlay.
/// 
/// Can be used in combination with the [`Card`](crate::native::card::Card)
/// widget to form dialog elements.
/// 
/// # Example
/// ```
/// # use iced_aw::native::modal;
/// # use iced_native::{Text, renderer::Null};
/// #
/// # pub type Modal<'a, S, Content, Message>
/// #  = iced_aw::native::Modal<'a, Message, S, Content, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     CloseModal,
/// }
/// 
/// let mut state = modal::State::new(());
/// 
/// let modal = Modal::new(
///     &mut state,
///     Text::new("Underlay"),
///     |_state| Text::new("Overlay").into()
/// )
/// .backdrop(Message::CloseModal);
/// ```
#[allow(missing_debug_implementations)]
pub struct Modal<'a, S, Content, Message, Renderer>
where
    S: 'a,
    Content: Fn(&mut S) -> Element<'_, Message, Renderer>,
    Message: Clone,
    Renderer: modal::Renderer,
{
    state: &'a mut State<S>,
    underlay: Element<'a, Message, Renderer>,
    content: Content,
    backdrop: Option<Message>,
    esc: Option<Message>,
    style: Renderer::Style,
}

impl<'a, S, Content, Message, Renderer> Modal<'a, S, Content, Message, Renderer>
where 
    S: 'a,
    Content: Fn(&mut S) -> Element<'_, Message, Renderer>,
    Message: Clone,
    Renderer: modal::Renderer,
{
    /// Creates a new [`Modal`](Modal) wrapping the underlying element to
    /// show some content as an overlay.
    /// 
    /// `state` is the content's state, assigned at the creation of the
    /// overlying content.
    pub fn new<U>(
        state: &'a mut State<S>,
        underlay: U,
        content: Content,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        Modal {
            state,
            underlay: underlay.into(),
            content,
            backdrop: None,
            esc: None,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the message that will be produced when the backdrop of the
    /// [`Modal`](Modal) is clicked.
    pub fn backdrop(mut self, message: Message) -> Self {
        self.backdrop = Some(message);
        self
    }

    /// Sets the message that will be produced when the Escape Key is
    /// pressed when the modal is open.
    /// 
    /// This can be used to close the modal on ESC.
    pub fn on_esc(mut self, message: Message) -> Self {
        self.esc = Some(message);
        self
    }

    /// Sets the style of the [`Modal`](Modal).
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

/// The state of the modal.
#[derive(Debug)]
pub struct State<S> {
    show: bool,
    state: S,
}

impl<S> State<S> {
    /// Creates a new [`State`](State) containing the given state data.
    pub fn new(s: S) -> Self {
        State {
            show: false,
            state: s,
        }
    }

    /// Setting this to true shows the modal (the modal is open), false means
    /// the modal is hidden (closed).
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }
}

impl<'a, S, Content, Message, Renderer> Widget<Message, Renderer>
    for Modal<'a, S, Content, Message, Renderer>
where
    S: 'a,
    Content: 'a + Fn(&mut S) -> Element<'_, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + modal::Renderer + iced_native::container::Renderer,
{
    fn width(&self) -> iced_native::Length {
        self.underlay.width()
    }

    fn height(&self) -> iced_native::Length {
        self.underlay.height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        self.underlay.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        self.underlay.draw(
            renderer,
            defaults,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.state.show.hash(state);
        self.underlay.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if !self.state.show { return self.underlay.overlay(layout); }

        let bounds = layout.bounds();
        let position = Point::new(bounds.x, bounds.y);

        Some(
            ModalOverlay::new(
                &mut self.state.state,
                &self.content,
                self.backdrop.clone(),
                self.esc.clone(),
                &self.style,
            )
            .overlay(position)
        )
    }
}

impl<'a, State, Content, Message, Renderer> From<Modal<'a, State, Content, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    State: 'a,
    Content: 'a + Fn(&mut State) -> Element<'_, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + modal::Renderer + iced_native::container::Renderer,
{
    fn from(modal: Modal<'a, State, Content, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}