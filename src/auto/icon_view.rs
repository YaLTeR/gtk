// This file was generated by gir (c486be0) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use Orientation;
use Scrollable;
use SelectionMode;
use TreeModel;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct IconView(Object<ffi::GtkIconView>): Widget, Container, Buildable, Scrollable;

    match fn {
        get_type => || ffi::gtk_icon_view_get_type(),
    }
}

impl IconView {
    pub fn new() -> IconView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_area<T: Upcast</*Ignored*/CellArea>>(area: &T) -> IconView {
    //    unsafe { TODO: call ffi::gtk_icon_view_new_with_area() }
    //}

    pub fn new_with_model<T: Upcast<TreeModel>>(model: &T) -> IconView {
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        unsafe {
            let mut bx = mem::uninitialized();
            let mut by = mem::uninitialized();
            ffi::gtk_icon_view_convert_widget_to_bin_window_coords(self.to_glib_none().0, wx, wy, &mut bx, &mut by);
            (bx, by)
        }
    }

    //pub fn create_drag_icon(&self, path: /*Ignored*/&TreePath) -> /*Ignored*/cairo::Surface {
    //    unsafe { TODO: call ffi::gtk_icon_view_create_drag_icon() }
    //}

    //pub fn enable_model_drag_dest(&self, targets: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 1, id: 238 }", n_targets: i32, actions: gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_icon_view_enable_model_drag_dest() }
    //}

    //pub fn enable_model_drag_source(&self, start_button_mask: gdk::ModifierType, targets: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 1, id: 238 }", n_targets: i32, actions: gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_icon_view_enable_model_drag_source() }
    //}

    #[cfg(gtk_3_8)]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    //#[cfg(gtk_3_6)]
    //pub fn get_cell_rect<T: Upcast</*Ignored*/CellRenderer>>(&self, path: /*Ignored*/&TreePath, cell: Option<&T>) -> Option<Rectangle> {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_cell_rect() }
    //}

    pub fn get_column_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_column_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_columns(self.to_glib_none().0)
        }
    }

    //pub fn get_cursor<T: Upcast</*Ignored*/CellRenderer>>(&self, path: /*Ignored*/TreePath, cell: &T) -> bool {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_cursor() }
    //}

    //pub fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32, path: /*Ignored*/TreePath) -> Option<IconViewDropPosition> {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_dest_item_at_pos() }
    //}

    //pub fn get_drag_dest_item(&self, path: /*Ignored*/TreePath) -> IconViewDropPosition {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_drag_dest_item() }
    //}

    //pub fn get_item_at_pos<T: Upcast</*Ignored*/CellRenderer>>(&self, x: i32, y: i32, path: /*Ignored*/TreePath, cell: &T) -> bool {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_item_at_pos() }
    //}

    //pub fn get_item_column(&self, path: /*Ignored*/&TreePath) -> i32 {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_item_column() }
    //}

    pub fn get_item_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_icon_view_get_item_orientation(self.to_glib_none().0)
        }
    }

    pub fn get_item_padding(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_padding(self.to_glib_none().0)
        }
    }

    //pub fn get_item_row(&self, path: /*Ignored*/&TreePath) -> i32 {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_item_row() }
    //}

    pub fn get_item_width(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_width(self.to_glib_none().0)
        }
    }

    pub fn get_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_margin(self.to_glib_none().0)
        }
    }

    pub fn get_markup_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_markup_column(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_icon_view_get_model(self.to_glib_none().0))
        }
    }

    //pub fn get_path_at_pos(&self, x: i32, y: i32) -> /*Ignored*/TreePath {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_path_at_pos() }
    //}

    pub fn get_pixbuf_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_pixbuf_column(self.to_glib_none().0)
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_reorderable(self.to_glib_none().0))
        }
    }

    pub fn get_row_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_row_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            ffi::gtk_icon_view_get_selection_mode(self.to_glib_none().0)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_text_column(self.to_glib_none().0)
        }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_tooltip_column(self.to_glib_none().0)
        }
    }

    //pub fn get_tooltip_context<T: Upcast</*Ignored*/TreeModel>>(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool, model: &T, path: /*Ignored*/TreePath, iter: /*Ignored*/TreeIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_tooltip_context() }
    //}

    //pub fn get_visible_range(&self, start_path: /*Ignored*/TreePath, end_path: /*Ignored*/TreePath) -> bool {
    //    unsafe { TODO: call ffi::gtk_icon_view_get_visible_range() }
    //}

    //pub fn item_activated(&self, path: /*Ignored*/&TreePath) {
    //    unsafe { TODO: call ffi::gtk_icon_view_item_activated() }
    //}

    //pub fn path_is_selected(&self, path: /*Ignored*/&TreePath) -> bool {
    //    unsafe { TODO: call ffi::gtk_icon_view_path_is_selected() }
    //}

    //pub fn scroll_to_path(&self, path: /*Ignored*/&TreePath, use_align: bool, row_align: f32, col_align: f32) {
    //    unsafe { TODO: call ffi::gtk_icon_view_scroll_to_path() }
    //}

    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_icon_view_select_all(self.to_glib_none().0);
        }
    }

    //pub fn select_path(&self, path: /*Ignored*/&TreePath) {
    //    unsafe { TODO: call ffi::gtk_icon_view_select_path() }
    //}

    //pub fn selected_foreach(&self, func: /*Unknown conversion*/Unknown rust type: "IconViewForeachFunc", data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_icon_view_selected_foreach() }
    //}

    #[cfg(gtk_3_8)]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_icon_view_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    pub fn set_column_spacing(&self, column_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_column_spacing(self.to_glib_none().0, column_spacing);
        }
    }

    pub fn set_columns(&self, columns: i32) {
        unsafe {
            ffi::gtk_icon_view_set_columns(self.to_glib_none().0, columns);
        }
    }

    //pub fn set_cursor<T: Upcast</*Ignored*/CellRenderer>>(&self, path: /*Ignored*/&TreePath, cell: Option<&T>, start_editing: bool) {
    //    unsafe { TODO: call ffi::gtk_icon_view_set_cursor() }
    //}

    //pub fn set_drag_dest_item(&self, path: /*Ignored*/Option<&TreePath>, pos: IconViewDropPosition) {
    //    unsafe { TODO: call ffi::gtk_icon_view_set_drag_dest_item() }
    //}

    pub fn set_item_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_icon_view_set_item_orientation(self.to_glib_none().0, orientation);
        }
    }

    pub fn set_item_padding(&self, item_padding: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_padding(self.to_glib_none().0, item_padding);
        }
    }

    pub fn set_item_width(&self, item_width: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_width(self.to_glib_none().0, item_width);
        }
    }

    pub fn set_margin(&self, margin: i32) {
        unsafe {
            ffi::gtk_icon_view_set_margin(self.to_glib_none().0, margin);
        }
    }

    pub fn set_markup_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_markup_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_icon_view_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_pixbuf_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_pixbuf_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_icon_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    pub fn set_row_spacing(&self, row_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_row_spacing(self.to_glib_none().0, row_spacing);
        }
    }

    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_icon_view_set_selection_mode(self.to_glib_none().0, mode);
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_text_column(self.to_glib_none().0, column);
        }
    }

    //pub fn set_tooltip_cell<T: Upcast</*Ignored*/CellRenderer>>(&self, tooltip: /*Ignored*/&Tooltip, path: /*Ignored*/&TreePath, cell: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_icon_view_set_tooltip_cell() }
    //}

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_column(self.to_glib_none().0, column);
        }
    }

    //pub fn set_tooltip_item(&self, tooltip: /*Ignored*/&Tooltip, path: /*Ignored*/&TreePath) {
    //    unsafe { TODO: call ffi::gtk_icon_view_set_tooltip_item() }
    //}

    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_icon_view_unselect_all(self.to_glib_none().0);
        }
    }

    //pub fn unselect_path(&self, path: /*Ignored*/&TreePath) {
    //    unsafe { TODO: call ffi::gtk_icon_view_unselect_path() }
    //}

    pub fn unset_model_drag_dest(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_dest(self.to_glib_none().0);
        }
    }

    pub fn unset_model_drag_source(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_source(self.to_glib_none().0);
        }
    }

}
