use std::collections::HashMap;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::platform::windows::WindowExtWindows;
use glutin::window::{Window, WindowId};
use image::{EncodableLayout, open};
use winapi::shared::minwindef::{FALSE, INT};
use winapi::um::combaseapi::CreateStreamOnHGlobal;
use winapi::um::gdiplusflat::{GdipCreateFromHWND, GdipDeleteGraphics, GdipDrawImageRectI, GdipLoadImageFromFile, GdipLoadImageFromStream};
use winapi::um::gdiplusinit::{GdiplusStartup, GdiplusStartupInput, GdiplusStartupOutput};
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};

#[macro_export]
macro_rules! wchar {
    ($str:expr) => {
        OsStr::new($str).encode_wide().chain(once(0)).collect::<Vec<u16>>().as_ptr()
    };
}
fn main() {
    let el = EventLoop::new();
    let mut windows = HashMap::new();
    let win = Window::new(&el).unwrap();
    windows.insert(1, win);

    el.run(move |event, event_loop, control_flow| unsafe {
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CloseRequested => { windows.remove(&1); }
                    _ => {}
                }
            }
            Event::RedrawRequested(id) => draw(windows.get_mut(&1).unwrap(), id),
            Event::LoopDestroyed => return,
            _ => {}
        }
        if unsafe { windows.is_empty() } {
            *control_flow = ControlFlow::Exit
        } else {
            *control_flow = ControlFlow::Wait
        }
    });
}

unsafe fn draw(window: &mut Window, id: WindowId) {
    let mut graphics = null_mut();
    let mut token = 0;
    GdiplusStartup(&mut token, &mut GdiplusStartupInput {
        GdiplusVersion: 1,
        DebugEventCallback: None,
        SuppressBackgroundThread: 0,
        SuppressExternalCodecs: 0,
    }, &mut GdiplusStartupOutput { NotificationHook: None, NotificationUnhook: None });
    GdipCreateFromHWND(window.hwnd() as winapi::shared::windef::HWND, &mut graphics);

    // data
    let image = open("img.png").unwrap();
    let data = image.as_bytes();

    // create steam
    let mut stream = null_mut();
    let hglobal = GlobalAlloc(GMEM_MOVEABLE, data.len());
    println!("GlobalAlloc - {:?}", hglobal);
    let mut buffer = GlobalLock(hglobal) as *mut u8;
    buffer.copy_from_nonoverlapping(data.as_ptr(), data.len());
    GlobalUnlock(hglobal);

    let i = CreateStreamOnHGlobal(hglobal, FALSE, &mut stream);
    println!("CreateStreamOnHGlobal - {} stream - {:?}", i, stream);
    GlobalFree(hglobal);


    // load image
    let mut gp_image = null_mut();

    let i = GdipLoadImageFromStream(stream, &mut gp_image);
    println!("GdipLoadImageFromStream - {}", i);

    let i = GdipLoadImageFromFile(wchar!("img.png"), &mut gp_image);
    println!("GdipLoadImageFromFile - {}", i);
    let i = GdipDrawImageRectI(graphics, gp_image, 0, 0, window.inner_size().width as INT, window.inner_size().height as INT);
    println!("GdipDrawImageRectI - {}", i);


    GdipDeleteGraphics(graphics);
}