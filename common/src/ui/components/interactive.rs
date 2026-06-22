//! `Interactive` — the custom animated `Widget` every control is built on.
//!
//! It wraps any content and owns its own hover/press animation state in the widget
//! tree (so per-component micro-interactions never touch app state). It draws an
//! interpolated **fill** behind the content (fill-over-outline: no rest border) and
//! an optional **ring** for selection/focus, lifts the content on hover/press, and
//! self-drives redraws via `Shell` while a tween is in flight.

use crate::ui::motion;
use crate::ui::theme::mix;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Operation, Tree, tree};
use iced::advanced::{Clipboard, Shell, Widget};
use iced::event::Event;
use iced::mouse;
use iced::time::Instant;
use iced::touch;
use iced::window;
use iced::{
    Animation, Background, Border, Color, Element, Length, Padding, Rectangle, Shadow, Size, Vector,
};

/// The visual of the surface at one state.
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    /// Fill color (use a 0-alpha color for "no fill").
    pub fill: Color,
    /// Vertical offset in px applied to the content (negative = lift up).
    pub lift: f32,
}

impl Surface {
    pub const fn new(fill: Color, lift: f32) -> Self {
        Self { fill, lift }
    }
}

/// The rest → hover → pressed ramp the widget interpolates across.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub rest: Surface,
    pub hover: Surface,
    pub pressed: Surface,
    pub radius: f32,
    /// Optional ring drawn over the fill — a state cue (selection / focus), the one
    /// sanctioned border. `None` means no border at all.
    pub ring: Option<(Color, f32)>,
}

impl Style {
    /// A surface whose fill is the same in every state (no hover/press change).
    pub fn flat(fill: Color, radius: f32) -> Self {
        let s = Surface::new(fill, 0.0);
        Self {
            rest: s,
            hover: s,
            pressed: s,
            radius,
            ring: None,
        }
    }

    pub fn with_ring(mut self, ring: Option<(Color, f32)>) -> Self {
        self.ring = ring;
        self
    }
}

#[derive(Debug)]
struct State {
    hover: Animation<bool>,
    press: Animation<bool>,
}

impl State {
    fn new() -> Self {
        Self {
            hover: Animation::new(false).easing(motion::EASING).duration(motion::QUICK),
            press: Animation::new(false).easing(motion::EASING).duration(motion::PRESS),
        }
    }
}

/// An animated, pressable surface wrapping `content`.
pub struct Interactive<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<Message>,
    style: Style,
    padding: Padding,
    width: Length,
    height: Length,
    reduce_motion: bool,
}

impl<'a, Message, Theme, Renderer> Interactive<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>, style: Style) -> Self {
        Self {
            content: content.into(),
            on_press: None,
            style,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            reduce_motion: motion::reduce_motion(),
        }
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn reduce_motion(mut self, reduce: bool) -> Self {
        self.reduce_motion = reduce;
        self
    }

    fn progress(&self, anim: &Animation<bool>, now: Instant) -> f32 {
        if self.reduce_motion {
            if anim.value() { 1.0 } else { 0.0 }
        } else {
            anim.interpolate(0.0, 1.0, now)
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Interactive<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(limits, self.width, self.height, self.padding, |limits| {
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        })
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        let now = Instant::now();
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() && is_over {
                    state.press.go_mut(true, now);
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if state.press.value() {
                    state.press.go_mut(false, now);
                    shell.request_redraw();
                    if is_over {
                        if let Some(on_press) = &self.on_press {
                            shell.publish(on_press.clone());
                        }
                    }
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.press.go_mut(false, now);
                shell.request_redraw();
            }
            _ => {}
        }

        if is_over != state.hover.value() {
            state.hover.go_mut(is_over, now);
            shell.request_redraw();
        }

        if let Event::Window(window::Event::RedrawRequested(_)) = event {
            if state.hover.is_animating(now) || state.press.is_animating(now) {
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let now = Instant::now();
        let state = tree.state.downcast_ref::<State>();
        let hp = self.progress(&state.hover, now);
        let pp = self.progress(&state.press, now);

        let fill = mix(
            mix(self.style.rest.fill, self.style.hover.fill, hp),
            self.style.pressed.fill,
            pp,
        );
        let lift = lerp(
            lerp(self.style.rest.lift, self.style.hover.lift, hp),
            self.style.pressed.lift,
            pp,
        );

        let border = match self.style.ring {
            Some((color, width)) => Border {
                color,
                width,
                radius: self.style.radius.into(),
            },
            None => Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: self.style.radius.into(),
            },
        };

        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            Background::Color(fill),
        );

        let content_layout = layout.children().next().unwrap();
        renderer.with_translation(Vector::new(0.0, lift), |renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let content_interaction = self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        );

        if content_interaction != mouse::Interaction::None {
            content_interaction
        } else if self.on_press.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Interactive<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(widget: Interactive<'a, Message, Theme, Renderer>) -> Self {
        Self::new(widget)
    }
}
