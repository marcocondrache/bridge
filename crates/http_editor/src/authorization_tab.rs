use gpui::{
    AppContext, Context, Entity, ParentElement, Render, SharedString, Styled, Subscription,
    WeakEntity, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    input::{Input, InputState},
    select::{Select, SelectEvent, SelectItem},
    v_flex,
};

use crate::{
    HttpEditor,
    authorization_type_selector::{AuthorizationTypeSelector, authorization_type_selector},
};

#[derive(Debug, Default, Clone)]
pub enum AuthorizationType {
    #[default]
    None,
    Basic,
    Bearer,
}

impl From<AuthorizationType> for SharedString {
    fn from(kind: AuthorizationType) -> Self {
        match kind {
            AuthorizationType::None => "None".into(),
            AuthorizationType::Basic => "Basic".into(),
            AuthorizationType::Bearer => "Bearer".into(),
        }
    }
}

impl AuthorizationType {
    pub fn all() -> [Self; 3] {
        [
            AuthorizationType::None,
            AuthorizationType::Basic,
            AuthorizationType::Bearer,
        ]
    }
}

impl SelectItem for AuthorizationType {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        self.clone().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

enum ActiveView {
    None,
    Basic {
        username: Entity<InputState>,
        password: Entity<InputState>,
    },
    Bearer {
        token: Entity<InputState>,
    },
}

impl ActiveView {
    fn basic(window: &mut Window, cx: &mut Context<AuthorizationTab>) -> Self {
        let username = cx.new(|cx| InputState::new(window, cx));
        let password = cx.new(|cx| InputState::new(window, cx));

        Self::Basic { username, password }
    }

    fn bearer(window: &mut Window, cx: &mut Context<AuthorizationTab>) -> Self {
        let token = cx.new(|cx| InputState::new(window, cx));

        Self::Bearer { token }
    }
}

pub struct AuthorizationTab {
    editor: WeakEntity<HttpEditor>,
    selector: Entity<AuthorizationTypeSelector>,
    active_view: ActiveView,
    _selector_subscription: Subscription,
}

impl AuthorizationTab {
    pub fn new(
        editor: WeakEntity<HttpEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selector = cx.new(|cx| authorization_type_selector(window, cx));
        let _selector_subscription =
            cx.subscribe_in(&selector, window, |this, _state, event, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let view = match value {
                        AuthorizationType::None => ActiveView::None,
                        AuthorizationType::Basic => ActiveView::basic(window, cx),
                        AuthorizationType::Bearer => ActiveView::bearer(window, cx),
                    };

                    this.set_active_view(view, window, cx);
                }
            });

        Self {
            selector,
            editor,
            active_view: ActiveView::None,
            _selector_subscription,
        }
    }

    fn set_active_view(
        &mut self,
        new_view: ActiveView,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_view = new_view;
    }

    fn render_selector(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div().child(Select::new(&self.selector))
    }
}

impl Render for AuthorizationTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .gap_4()
            .child(self.render_selector(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::None => parent.child("No authorization selected"),
                ActiveView::Basic { username, password } => parent
                    .child("Basic authorization")
                    .child(Input::new(&username))
                    .child(Input::new(&password)),
                ActiveView::Bearer { token } => parent
                    .child("Bearer authorization")
                    .child(Input::new(&token)),
            })
    }
}
