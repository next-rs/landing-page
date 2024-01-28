use crate::components::footer::Footer;
use crate::components::banner::Banner;
use next_rs::prelude::*;
use next_rs::{Menu, Navbar, NavbarProps};
use yew_scroll::ScrollToTop;

#[func]
pub fn LandingPage() -> Html {
    let menus: Vec<Menu> = vec![
        Menu {
            id: 1,
            link: "#about",
            name: "About",
        },
    ];

    let navbar_props = NavbarProps {
        menus: menus,
        button_text: "StarUs",
        logo_img_class: "w-20",
        button_href: "https://github.com/next-rs",
        button_link_class: "bg-purple-600 text-white px-4 py-2 rounded-full text-center",
        ..NavbarProps::default()
    };

    html! {
        <>
          <Navbar ..navbar_props />
          <Banner />
          <Footer />
          <ScrollToTop />
        </>
    }
}
