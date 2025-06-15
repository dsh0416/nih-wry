use std::{num::NonZero, sync::{Arc, RwLock}};

use nih_plug::{editor::{Editor, ParentWindowHandle}, prelude::GuiContext};
use raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};
use wry::{WebViewAttributes, WebViewBuilder};

pub(crate) struct WryEditor<T> {
    pub(crate) user_state: Arc<RwLock<T>>,
    pub(crate) url: String,
}

struct ParentWindowHandleAdapter(nih_plug::editor::ParentWindowHandle);

impl HasWindowHandle for ParentWindowHandleAdapter {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.0 {
            ParentWindowHandle::X11Window(window) => {
                let handle = raw_window_handle::XcbWindowHandle::new(NonZero::new(window).unwrap());
                Ok(unsafe { WindowHandle::borrow_raw(raw_window_handle::RawWindowHandle::Xcb(handle)) })
            }
            ParentWindowHandle::AppKitNsView(ns_view) => {
                let ns_view = std::ptr::NonNull::new(ns_view as *mut std::ffi::c_void).expect("msg: ns_view must not be null");
                let handle = raw_window_handle::AppKitWindowHandle::new(ns_view);
                Ok(unsafe { WindowHandle::borrow_raw(raw_window_handle::RawWindowHandle::AppKit(handle)) })
            }
            ParentWindowHandle::Win32Hwnd(hwnd) => {
                let hwnd_isize = hwnd as isize;
                let hwnd_nonzero = std::num::NonZeroIsize::new(hwnd_isize).expect("msg: hwnd must not be zero");
                let handle = raw_window_handle::Win32WindowHandle::new(hwnd_nonzero);
                Ok(unsafe { WindowHandle::borrow_raw(raw_window_handle::RawWindowHandle::Win32(handle)) })
            }
        }
    }
}

struct WryEditorHandle {
    webview: Arc<wry::WebView>,
}

impl<T> Editor for WryEditor<T>
where
    T: 'static + Send + Sync,
{
    fn spawn(
        &self,
        parent: ParentWindowHandle,
        context: Arc<dyn GuiContext>,
    ) -> Box<dyn std::any::Any + Send + 'static> {
       let webview = WebViewBuilder::with_attributes(
        WebViewAttributes {
            url: Some(self.url.clone()),
            devtools: true,
            ..Default::default()
        }
       ).build_as_child(&ParentWindowHandleAdapter(parent)).unwrap();

       todo!()
    }    
    fn size(&self) -> (u32, u32) {
        todo!()
    }
    
    fn set_scale_factor(&self, _factor: f32) -> bool {
        // Wry handles scale factor automatically, so we return false to indicate no change
        false
    }
    
    fn param_value_changed(&self, id: &str, normalized_value: f32) {
        todo!()
    }
    
    fn param_modulation_changed(&self, id: &str, modulation_offset: f32) {
        todo!()
    }
    
    fn param_values_changed(&self) {
        todo!()
    }
}
