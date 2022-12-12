use rocket::{get, State};
use tauri::{AppHandle, Wry, Manager};


#[get("/show")]              // <- route attribute
pub async fn show(app: &State<AppHandle<Wry>>) -> &'static str {  
  let binding = app.windows();
  let window = binding.get("main").unwrap();

  window.show();
  window.set_focus();
  "window shown"
}

#[get("/hide")]              // <- route attribute
pub fn hide(app: &State<AppHandle<Wry>>) -> &'static str {  
  let binding = app.windows();
  let window = binding.get("main").unwrap();
  
  window.hide();
  "window shown"
}

#[get("/quit")]              // <- route attribute
pub fn quit(app: &State<AppHandle<Wry>>) -> &'static str {  
  app.exit(0);
  "program exited"
}


#[get("/status")]              // <- route attribute
pub fn status() -> &'static str {  
  "alive"
}