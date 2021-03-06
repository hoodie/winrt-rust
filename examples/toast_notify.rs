// Windows Toast notifications in Rust!
//
// Use the following command to run this example:
// > cargo run --example toast_notify --features "windows-data windows-ui"

extern crate winrt;

use winrt::*;
use winrt::windows::data::xml::dom::*;
use winrt::windows::ui::notifications::*;

fn main() {
    let rt = RuntimeContext::init();
    run();
    rt.uninit();
}

fn run() { unsafe {
    // Get a toast XML template
    let mut toast_xml = ToastNotificationManager::get_template_content(ToastTemplateType_ToastText02).unwrap();

    // Fill in the text elements
    let mut toast_text_elements = toast_xml.get_elements_by_tag_name(&FastHString::new("text")).unwrap();
    
    toast_text_elements.item(0).unwrap().append_child(&*toast_xml.create_text_node(&FastHString::new("Hello from Rust!")).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();
    toast_text_elements.item(1).unwrap().append_child(&*toast_xml.create_text_node(&FastHString::new("This is some more text.")).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();

    // could use the following to get the XML code for the notification
    //println!("{}", toast_xml.query_interface::<IXmlNodeSerializer>().unwrap().get_xml().unwrap());

    // Create the toast and attach event listeners
    let toast = ToastNotification::create_toast_notification(&*toast_xml).unwrap();

    // Show the toast. Apparently the App ID does not need to be registered anywhere (contrary to official documentation).
    ToastNotificationManager::create_toast_notifier_with_id(&FastHString::new("myrustapp.exe")).unwrap().show(&*toast).unwrap();
}}
