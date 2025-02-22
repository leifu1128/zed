use gpui::{actions, AppContext, ClipboardItem, PromptLevel};
use system_specs::SystemSpecs;
use workspace::Workspace;

pub mod deploy_feedback_button;
pub mod feedback_modal;

actions!(feedback, [GiveFeedback, SubmitFeedback]);

mod system_specs;

actions!(
    zed,
    [
        CopySystemSpecsIntoClipboard,
        FileBugReport,
        RequestFeature,
        OpenZedRepo
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        feedback_modal::FeedbackModal::register(workspace, cx);
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, cx| {
                let specs = SystemSpecs::new(&cx).to_string();

                let prompt = cx.prompt(
                    PromptLevel::Info,
                    "Copied into clipboard",
                    Some(&specs),
                    &["OK"],
                );
                cx.spawn(|_, _cx| async move {
                    prompt.await.ok();
                })
                .detach();
                cx.write_to_clipboard(ClipboardItem::new(specs.clone()));
            })
            .register_action(|_, _: &RequestFeature, cx| {
                let url = "https://github.com/zed-industries/zed/issues/new?assignees=&labels=enhancement%2Ctriage&template=0_feature_request.yml";
                cx.open_url(url);
            })
            .register_action(move |_, _: &FileBugReport, cx| {
                let url = format!(
                    "https://github.com/zed-industries/zed/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml&environment={}",
                    urlencoding::encode(&SystemSpecs::new(&cx).to_string())
                );
                cx.open_url(&url);
            })
            .register_action(move |_, _: &OpenZedRepo, cx| {
                let url = "https://github.com/zed-industries/zed";
                cx.open_url(&url);
            });
        })
    .detach();
}
