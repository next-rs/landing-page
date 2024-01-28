use next_rs::prelude::*;
use next_rs::{Link, Image, ImageProps};

const CONTAINER_CLASS: &str = "container mx-auto px-4";
const FLEX_CONTAINER_CLASS: &str = "flex items-center justify-end w-full h-screen pt-10 bg-cover bg-center relative";
const LEFT_SECTION_CLASS: &str = "w-full md:w-1/2 flex flex-col items-center justify-center md:items-start md:py-20";
const BANNER_TITLE_CLASS: &str = "text-5xl md:text-7xl font-bold animate-gradient pb-4";
const BANNER_SUBTITLE_CLASS: &str = "pl-2 max-w-md text-lg md:text-xl text-white font-bold";
const FLEX_CLASS: &str = "flex mx-4 my-5";
const CUSTOM_BUTTON_CLASS: &str = "bg-purple-600 text-white px-4 py-2 rounded-full text-center";
const RIGHT_SECTION_CLASS: &str = "hidden md:flex md:w-1/2 w-full";
const RELATIVE_CLASS: &str = "relative text-center mt-3 p-md-2";
const BANNER_IMAGE_CLASS: &str = "w-full h-auto md:w-auto md:h-full banner-image";

#[func]
pub fn Banner() -> Html {
    html! {
        <section id="about" class={FLEX_CONTAINER_CLASS}>
            <div class={CONTAINER_CLASS}>
                <div class={FLEX_CONTAINER_CLASS}>
                    {left_section()}
                    {right_section()}
                </div>
            </div>
        </section>
    }
}

fn left_section() -> Html {
    html! {
        <div class={LEFT_SECTION_CLASS}>
            <h1 class={BANNER_TITLE_CLASS}>
                {"Coming Soon!"}
            </h1>
            <p class={BANNER_SUBTITLE_CLASS}>
                {"Build Type Safe Web Apps Blazingly Fast!"}
            </p>
            <div class={FLEX_CLASS}>
                <div>
                    <Link to={"https://github.com/next-rs"} class={CUSTOM_BUTTON_CLASS}>
                        {"Build Now"}
                    </Link>
                </div>
            </div>
        </div>
    }
}

fn right_section() -> Html {
    let image_props = ImageProps {
        src: "images/banner.png",
        alt: "Next RS Banner",
        width: "200",
        height: "300",
        style: "",
        class: BANNER_IMAGE_CLASS,
        sizes: "(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw",
        quality: "80",
        priority: true,
        placeholder: "blur",
        on_loading_complete: Callback::from(|_| {
            println!("Image loading is complete!");
        }),
        object_fit: "cover",
        object_position: "center",
        on_error: Callback::from(|err| {
            println!("Error loading image: {:?}", err);
        }),
        decoding: "async",
        blur_data_url: "data:image/png;base64,....",
        lazy_boundary: "200px",
        unoptimized: false,
        node_ref: NodeRef::default(),
    };
    html! {
        <div class={RIGHT_SECTION_CLASS}>
            <div class={RELATIVE_CLASS}>
                <Image ..image_props />
            </div>
        </div>
    }
}
