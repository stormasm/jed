//mod base_keymap_picker;
//mod base_keymap_setting;

//use client::{telemetry::Telemetry, TelemetrySettings};
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    svg, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    ParentElement, Render, Styled, Subscription, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use settings::SettingsStore;
use std::sync::Arc;
use ui::prelude::*;
//use vim::VimModeSetting;
use workspace::{
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, AppState, Welcomg, Workspace, WorkspaceId,
};

//pub use base_keymap_setting::BaseKeymap;

pub const FIRST_OPEN: &str = "first_open";

pub fn init(cx: &mut AppContext) {
    // BaseKeymap::register(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &Welcomg, cx| {
            let welcome_page = WelcomePage::new(workspace, cx);
            workspace.add_item(Box::new(welcome_page), cx)
        });
    })
    .detach();

    //base_keymap_picker::init(cx);
}

pub fn show_welcome_view(app_state: &Arc<AppState>, cx: &mut AppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
        let welcome_page = WelcomePage::new(workspace, cx);
        workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
        cx.focus_view(&welcome_page);
        cx.notify();
    })
    .detach();

    db::write_and_log(cx, || {
        KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
    });
}

pub struct WelcomePage {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    //telemetry: Arc<Telemetry>,
    _settings_subscription: Subscription,
}

impl Render for WelcomePage {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .full()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .w_96()
                    .gap_4()
                    .mx_auto()
                    .child(
                        svg()
                            .path("icons/logo_96.svg")
                            .text_color(gpui::white())
                            .w(px(96.))
                            .h(px(96.))
                            .mx_auto(),
                    )
                    .child(h_flex().justify_center().child(Label::new("Hello World")))
                    .child(
                        v_flex()
                            .p_3()
                            .gap_2()
                            .bg(cx.theme().colors().elevated_surface_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_md()
                            .child(h_flex().gap_2().child(Label::new("Send crash reports"))),
                    ),
            )
    }
}

impl WelcomePage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let this = cx.new_view(|cx| {
            /*
                        cx.on_release(|this: &mut Self, _, _| {
                            this.telemetry
                                .report_app_event("welcome page: close".to_string());
                        })
                        .detach();
            */
            WelcomePage {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                //telemetry: workspace.client().telemetry().clone(),
                _settings_subscription: cx
                    .observe_global::<SettingsStore>(move |_, cx| cx.notify()),
            }
        });

        this
    }

    /*
    fn update_settings<T: Settings>(
        &mut self,
        selection: &Selection,
        cx: &mut ViewContext<Self>,
        callback: impl 'static + Send + Fn(&mut T::FileContent, bool),
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            let selection = *selection;
            settings::update_settings_file::<T>(fs, cx, move |settings| {
                let value = match selection {
                    Selection::Unselected => false,
                    Selection::Selected => true,
                    _ => return,
                };

                callback(settings, value)
            });
        }
    }
    */
}

impl EventEmitter<ItemEvent> for WelcomePage {}

impl FocusableView for WelcomePage {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for WelcomePage {
    type Event = ItemEvent;

    fn tab_content(&self, _: Option<usize>, selected: bool, _: &WindowContext) -> AnyElement {
        Label::new("Welcomg to Jed!")
            .color(if selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("welcome page")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        Some(cx.new_view(|cx| WelcomePage {
            focus_handle: cx.focus_handle(),
            workspace: self.workspace.clone(),
            //telemetry: self.telemetry.clone(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
