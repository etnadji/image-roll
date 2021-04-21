use gtk::{prelude::BuilderExtManual, ApplicationWindow, Builder, GtkWindowExt, WidgetExt};

#[derive(Clone)]
pub struct Widgets {
    window: ApplicationWindow,
    open_menu_button: gtk::Button,
    image_widget: gtk::Image,
    popover_menu: gtk::PopoverMenu,
    next_button: gtk::Button,
    previous_button: gtk::Button,
    preview_smaller_button: gtk::Button,
    preview_larger_button: gtk::Button,
    image_viewport: gtk::Viewport,
    preview_size_combobox: gtk::ComboBoxText,
    image_event_box: gtk::EventBox,
    rotate_counterclockwise_button: gtk::Button,
    rotate_clockwise_button: gtk::Button,
    crop_button: gtk::ToggleButton,
    resize_button: gtk::MenuButton,
    width_spin_button: gtk::SpinButton,
    height_spin_button: gtk::SpinButton,
    link_aspect_ratio_button: gtk::ToggleButton,
    apply_resize_button: gtk::Button,
    error_info_bar: gtk::InfoBar,
    error_info_bar_text: gtk::Label,
    save_menu_button: gtk::Button,
}

impl Widgets {
    pub fn init(builder: Builder, application: &gtk::Application) -> Self {
        let window: ApplicationWindow = builder
            .get_object("main_window")
            .expect("Couldn't get main_window");
        window.set_application(Some(application));

        let open_menu_button: gtk::Button = builder
            .get_object("open_menu_button")
            .expect("Couldn't get open_menu_button");

        let image_widget: gtk::Image = builder
            .get_object("image_widget")
            .expect("Couldn't get image_widget");

        let popover_menu: gtk::PopoverMenu = builder
            .get_object("popover_menu")
            .expect("Couldn't get popover_menu");

        let next_button: gtk::Button = builder
            .get_object("next_button")
            .expect("Couldn't get next_button");
        let previous_button: gtk::Button = builder
            .get_object("previous_button")
            .expect("Couldn't get previous_button");

        let preview_smaller_button: gtk::Button = builder
            .get_object("preview_smaller_button")
            .expect("Couldn't get preview_smaller_button");
        let preview_larger_button: gtk::Button = builder
            .get_object("preview_larger_button")
            .expect("Couldn't get preview_larger_button");

        let image_viewport: gtk::Viewport = builder
            .get_object("image_viewport")
            .expect("Couldn't get image_viewport");

        let preview_size_combobox: gtk::ComboBoxText = builder
            .get_object("preview_size_combobox")
            .expect("Couldn't get preview_size_combobox");

        let image_event_box: gtk::EventBox = builder
            .get_object("image_event_box")
            .expect("Couldn't get image_preview_box");

        let rotate_counterclockwise_button: gtk::Button = builder
            .get_object("rotate_counterclockwise_button")
            .expect("Couldn't get rotate_counterclockwise_button");
        let rotate_clockwise_button: gtk::Button = builder
            .get_object("rotate_clockwise_button")
            .expect("Couldn't get rotate_clockwise_button");

        let crop_button: gtk::ToggleButton = builder
            .get_object("crop_button")
            .expect("Couldn't get crop_button");

        let resize_button: gtk::MenuButton = builder
            .get_object("resize_button")
            .expect("Couldn't get resize_button");
        resize_button.set_sensitive(false);

        let width_spin_button: gtk::SpinButton = builder
            .get_object("width_spin_button")
            .expect("Couldn't get width_spin_button");
        let height_spin_button: gtk::SpinButton = builder
            .get_object("height_spin_button")
            .expect("Couldn't get height_spin_button");

        let link_aspect_ratio_button: gtk::ToggleButton = builder
            .get_object("link_aspect_ratio_button")
            .expect("Couldn't get link_aspect_ratio_button");

        let apply_resize_button: gtk::Button = builder
            .get_object("apply_resize_button")
            .expect("Couldn't get apply_resize_button");

        let error_info_bar: gtk::InfoBar = builder
            .get_object("error_info_bar")
            .expect("Couldn't get error_info_bar");

        let error_info_bar_text: gtk::Label = builder
            .get_object("error_info_bar_text")
            .expect("Couldn't get error_info_bar_text");

        let save_menu_button: gtk::Button = builder
            .get_object("save_menu_button")
            .expect("Couldn't get save_menu_button");

        Self {
            window,
            open_menu_button,
            image_widget,
            popover_menu,
            next_button,
            previous_button,
            preview_smaller_button,
            preview_larger_button,
            image_viewport,
            preview_size_combobox,
            image_event_box,
            rotate_counterclockwise_button,
            rotate_clockwise_button,
            crop_button,
            resize_button,
            width_spin_button,
            height_spin_button,
            link_aspect_ratio_button,
            apply_resize_button,
            error_info_bar,
            error_info_bar_text,
            save_menu_button,
        }
    }

    /// Get a reference to the widgets's window.
    pub fn window(&self) -> &ApplicationWindow {
        &self.window
    }

    /// Get a reference to the widgets's open menu button.
    pub fn open_menu_button(&self) -> &gtk::Button {
        &self.open_menu_button
    }

    /// Get a reference to the widgets's image widget.
    pub fn image_widget(&self) -> &gtk::Image {
        &self.image_widget
    }

    /// Get a reference to the widgets's popover menu.
    pub fn popover_menu(&self) -> &gtk::PopoverMenu {
        &self.popover_menu
    }

    /// Get a reference to the widgets's next button.
    pub fn next_button(&self) -> &gtk::Button {
        &self.next_button
    }

    /// Get a reference to the widgets's previous button.
    pub fn previous_button(&self) -> &gtk::Button {
        &self.previous_button
    }

    /// Get a reference to the widgets's preview smaller button.
    pub fn preview_smaller_button(&self) -> &gtk::Button {
        &self.preview_smaller_button
    }

    /// Get a reference to the widgets's preview larger button.
    pub fn preview_larger_button(&self) -> &gtk::Button {
        &self.preview_larger_button
    }

    /// Get a reference to the widgets's image viewport.
    pub fn image_viewport(&self) -> &gtk::Viewport {
        &self.image_viewport
    }

    /// Get a reference to the widgets's preview size combobox.
    pub fn preview_size_combobox(&self) -> &gtk::ComboBoxText {
        &self.preview_size_combobox
    }

    /// Get a reference to the widgets's image event box.
    pub fn image_event_box(&self) -> &gtk::EventBox {
        &self.image_event_box
    }

    /// Get a reference to the widgets's rotate counterclockwise button.
    pub fn rotate_counterclockwise_button(&self) -> &gtk::Button {
        &self.rotate_counterclockwise_button
    }

    /// Get a reference to the widgets's rotate clockwise button.
    pub fn rotate_clockwise_button(&self) -> &gtk::Button {
        &self.rotate_clockwise_button
    }

    /// Get a reference to the widgets's crop button.
    pub fn crop_button(&self) -> &gtk::ToggleButton {
        &self.crop_button
    }

    /// Get a reference to the widgets's resize button.
    pub fn resize_button(&self) -> &gtk::MenuButton {
        &self.resize_button
    }

    /// Get a reference to the widgets's width spin button.
    pub fn width_spin_button(&self) -> &gtk::SpinButton {
        &self.width_spin_button
    }

    /// Get a reference to the widgets's height spin button.
    pub fn height_spin_button(&self) -> &gtk::SpinButton {
        &self.height_spin_button
    }

    /// Get a reference to the widgets's link aspect ratio button.
    pub fn link_aspect_ratio_button(&self) -> &gtk::ToggleButton {
        &self.link_aspect_ratio_button
    }

    /// Get a reference to the widgets's apply resize button.
    pub fn apply_resize_button(&self) -> &gtk::Button {
        &self.apply_resize_button
    }

    /// Get a reference to the widgets's error info bar.
    pub fn error_info_bar(&self) -> &gtk::InfoBar {
        &self.error_info_bar
    }

    /// Get a reference to the widgets's error info bar text.
    pub fn error_info_bar_text(&self) -> &gtk::Label {
        &self.error_info_bar_text
    }

    /// Get a reference to the widgets's save menu button.
    pub fn save_menu_button(&self) -> &gtk::Button {
        &self.save_menu_button
    }
}