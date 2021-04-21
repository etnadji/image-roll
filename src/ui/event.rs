use gdk::Rectangle;
use std::path::PathBuf;

use crate::{image::PreviewSize, image_operation::ImageOperation};

#[derive(Debug)]
pub enum Event {
    OpenFile(gio::File),
    LoadImage(Option<PathBuf>),
    ImageViewportResize(Rectangle),
    RefreshPreview(PreviewSize),
    ChangePreviewSize(PreviewSize),
    ImageEdit(ImageOperation),
    StartSelection((i32, i32)),
    DragSelection((i32, i32)),
    EndSelection,
    PreviewSmaller,
    PreviewLarger,
    NextImage,
    PreviousImage,
    RefreshFileList,
    FileListChanged,
    ResizePopoverDisplayed,
    UpdateResizePopoverWidth,
    UpdateResizePopoverHeight,
    SaveCurrentImage,
    DisplayError(anyhow::Error),
    HideErrorPanel,
}

pub fn post_event(sender: &glib::Sender<Event>, action: Event) {
    if let Err(err) = sender.send(action) {
        error!("Send error: {}", err);
    }
}