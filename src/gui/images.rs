use std::collections::HashMap;

use notan::prelude::{Graphics, Texture};

pub fn load_images(gfx: &mut Graphics) -> HashMap<String, Texture> {
    let mut image_data_list: HashMap<String, &'static [u8]> = HashMap::new();
    image_data_list.insert(String::from("shrine"), include_bytes!("./images/shrine.png"));
    image_data_list.insert(String::from("well"), include_bytes!("./images/well.png"));
    image_data_list.insert(String::from("chest"), include_bytes!("./images/chest.png"));
    image_data_list.insert(String::from("superchest"), include_bytes!("./images/superchest.png"));

    image_data_list.insert(String::from("key"), include_bytes!("./images/key.png"));
    image_data_list.insert(String::from("identify_scroll"), include_bytes!("./images/identify_scroll.png"));
    image_data_list.insert(String::from("town_portal_scroll"), include_bytes!("./images/town_portal_scroll.png"));

    

    let mut images: HashMap<String, Texture> = HashMap::new();

    for (name, bytes) in image_data_list.iter() {
        images.insert(name.to_string(), gfx.create_texture().from_image(*bytes).build().unwrap());
    }
    images
}
