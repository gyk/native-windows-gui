/*!
    An application that show how to use the TreeView control.

    Requires the following features: `cargo run --example treeview_d --features "tree-view tree-view-iterator listbox image-list frame"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct TreeViewApp {
    #[nwg_control(size: (600, 350), position: (300, 300), title: "TreeView - Musteloidea")]
    #[nwg_events( OnWindowClose: [TreeViewApp::exit], OnInit: [TreeViewApp::load_data] )]
    window: nwg::Window,

    #[nwg_resource(initial: 5, size: (16, 16))]
    view_icons: nwg::ImageList,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 3, row: 0, row_span: 6)]
    tree_view: nwg::TreeView,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 3, col_span: 2, row: 0, row_span: 3,)]
    control_frame: nwg::Frame,

    #[nwg_layout(parent: control_frame, spacing: 3, margin: [0,0,0,0])]
    control_layout: nwg::GridLayout,

    #[nwg_control(parent: control_frame, text: "Options:")]
    #[nwg_layout_item(layout: control_layout, col: 0, row: 0)]
    label1: nwg::Label,

    #[nwg_control(parent: control_frame, text: "New item name")]
    #[nwg_layout_item(layout: control_layout, col: 0, col_span: 2, row: 1)]
    new_item: nwg::TextInput,

    #[nwg_control(parent: control_frame, text: "Add")]
    #[nwg_layout_item(layout: control_layout, col: 0, row: 2)]
    add_btn: nwg::Button,

    #[nwg_control(parent: control_frame, text: "Del")]
    #[nwg_layout_item(layout: control_layout, col: 1, row: 2)]
    remove_btn: nwg::Button,

    #[nwg_control(text: "Events:")]
    #[nwg_layout_item(layout: layout, col: 3, col_span: 2, row: 3)]
    label2: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 3, col_span: 2, row: 4, row_span: 2)]
    events_log: nwg::ListBox<String>,
}

impl TreeViewApp {
    
    fn load_data(&self) {
        let tv = &self.tree_view;
        let icons = &self.view_icons;

        icons.add_icon_from_filename("./test_rc/cog.ico").unwrap();
        icons.add_icon_from_filename("./test_rc/love.ico").unwrap();

        tv.set_image_list(Some(icons));

        let root = tv.insert_item("Caniformia", None, nwg::TreeInsert::Root);
        tv.insert_item("Canidae (dogs and other canines)", Some(&root), nwg::TreeInsert::Last);
        
        let arc = tv.insert_item("Arctoidea", Some(&root), nwg::TreeInsert::Last);
        tv.insert_item("Ursidae (bears)", Some(&arc), nwg::TreeInsert::Last);
        
        let mus = tv.insert_item("Musteloidea (weasel)", Some(&arc), nwg::TreeInsert::Last);

        tv.insert_item("Mephitidae (skunks)", Some(&mus), nwg::TreeInsert::Last);
        tv.insert_item("Ailuridae (red panda)", Some(&mus), nwg::TreeInsert::Last);
        tv.insert_item("Procyonidae (raccoons and allies)", Some(&mus), nwg::TreeInsert::Last);
        tv.insert_item("Mustelidae (weasels and allies)", Some(&mus), nwg::TreeInsert::Last);

        tv.set_text_color(50, 50, 200);

        // Expand and sets the selected icon for each item in the tree
        for item in tv.iter() {
            tv.set_expand_state(&item, nwg::ExpandState::Expand);
            tv.set_item_image(&item, 1, true);
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = TreeViewApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
