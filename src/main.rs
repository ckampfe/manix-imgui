use std::{
    io::{Read, Write},
    path::Path,
};

use imgui::*;

mod clipboard;
mod support;

struct State<'a> {
    torrent_loaded: bool,
    foo: bool,
    error: String,
    client: manix::blocking_client::BlockingClient,
    current_item: i32,
    torrent_names: Vec<&'a [u8]>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = support::init(file!());

    let options = manix::Options::default();
    let client = manix::blocking_client(options);

    let mut state = State {
        torrent_loaded: false,
        foo: false,
        error: "".to_string(),
        client,
        current_item: 0,
        torrent_names: vec![],
    };

    system.main_loop(move |_, ui| {
        ui.main_menu_bar(|| {
            if let Some(menu) = ui.begin_menu(im_str!("foo"), true) {
                MenuItem::new(im_str!("something")).build_with_ref(ui, &mut state.foo);
                menu.end(ui)
            };
        });

        Window::new(im_str!("Command"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .menu_bar(true)
            .build(ui, || {
                if ui.button(im_str!("load test torrent"), [40.0, 20.0]) {
                    let path = Path::new("/home/clark/code/personal/manix/Root_Vase.stl.torrent");
                    let r = std::fs::File::open(path).unwrap();
                    let out: Box<dyn manix::RW> = Box::new(MyCursor(std::io::Cursor::new(vec![])));
                    let res = state.client.add_torrent(r, out);

                    // let result = manix.add_torrent(path);
                    // match result {
                    //     Ok(torrent) => {
                    //         state.torrent = Some(torrent);
                    //     }
                    //     Err(e) => {
                    //         state.error = e.to_string();
                    //     }
                    // }

                    state.torrent_loaded = true;
                }

                // if let Some(torrent) = &state.torrent {
                //     if ui.button(im_str!("announce"), [70.0, 20.0]) {}
                //     ui.text(&torrent.get_announce_url());
                //     ui.text(&torrent.get_info_hash_human())
                // } else {
                //     ui.text(&state.error)
                // }

                let torrents = state.client.list_torrents();
                let torrent_names: Vec<ImString> = torrents
                    .iter()
                    .map(|t| {
                        ImString::from(t.get_info_hash_human())
                        // t.get_info_hash_human().as_bytes()
                    })
                    .collect();

                let borrowed: Vec<&ImString> = torrent_names.iter().collect();

                ui.list_box(
                    im_str!("torrents"),
                    &mut state.current_item,
                    &borrowed,
                    // state.torrent_names.len() as i32,
                    8,
                );
            })
    });

    Ok(())
}
struct MyCursor(std::io::Cursor<Vec<u8>>);

impl manix::RW for MyCursor {}

impl Read for MyCursor {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl Write for MyCursor {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
