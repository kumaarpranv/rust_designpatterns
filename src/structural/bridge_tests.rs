use crate::structural::bridge::{Radio, Tv, RemoteControl};

#[test]
fn test_radio_remote() {
    let radio = Radio::new();
    let tv = Tv::new();

    let mut radio_remote = RemoteControl::new(Box::new(radio));
    let mut tv_remote = RemoteControl::new(Box::new(tv));

    radio_remote.toggle_power();
    tv_remote.toggle_power();

    radio_remote.volume_up();
    tv_remote.volume_down();
}

