//! Icons I robbed from [Hero Icons](https://heroicons.com)
//! I stole this from another project I did a while back, so there may be unused icons here :)

use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum IconType {
    Cog,
    User,
    Hamburger,
    LightBulbLight,
    LightBulbDark,
}
impl IconType {
    pub fn convert_to_svg(&self) -> Html {
        use IconType::*;
        match *self {
            User => html! {
                // `user`
                <svg
                    xmlns={"http://www.w3.org/2000/svg"}
                    fill={"none"} viewBox={"0 0 24 24"}
                    stroke-width={"1.5"}
                    stroke={"currentColor"}
                >
                    <path
                        stroke-linecap={"round"}
                        stroke-linejoin={"round"}
                        d={"M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933\
                                0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"}
                    />
                </svg>
            },
            Cog => html! {
                // `cog-8-tooth`
                <svg
                    xmlns={"http://www.w3.org/2000/svg"}
                    fill={"none"}
                    viewBox={"0 0 24 24"}
                    stroke-width={"1.5"}
                    stroke={"currentColor"}
                 >
                    <path
                        stroke-linecap={"round"}
                        stroke-linejoin={"round"}
                        d={"M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 \
                                1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 \
                                1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 \
                                1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 \
                                1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781\
                                .929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93\
                                -.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 \
                                01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v\
                                -1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 \
                                1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l\
                                .15-.894z"}
                    />
                    <path
                        stroke-linecap={"round"}
                        stroke-linejoin={"round"}
                        d={"M15 12a3 3 0 11-6 0 3 3 0 016 0z"}
                    />
                </svg>
            },
            Hamburger => html! {
                <svg
                    xmlns={"http://www.w3.org/2000/svg"}
                    fill={"none"}
                    viewBox={"0 0 24 24"}
                    stroke-width={"1.5"}
                    stroke={"currentColor"}
                >
                    <path
                        stroke-linecap={"round"}
                        stroke-linejoin={"round"}
                        d={"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"}
                    />
                </svg>
            },
            LightBulbLight => html! {
                <svg
                    xmlns={"http://www.w3.org/2000/svg"}
                    fill={"none"}
                    viewBox={"0 0 24 24"}
                    stroke-width={1.5}
                    stroke={"currentColor"}
                >
                    <path
                        stroke-linecap={"round"}
                        stroke-linejoin={"round"}
                        d={"M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18" }
                     />
                 </svg>
            },
            LightBulbDark => html! {
                <svg
                    xmlns={"http://www.w3.org/2000/svg"}
                    viewBox={"0 0 24 24"}
                    fill={"currentColor"}
                >
                    <path
                        d={"M12 .75a8.25 8.25 0 0 0-4.135 15.39c.686.398 1.115 1.008 1.134 1.623a.75.75 0 0 0 \
                            .577.706c.352.083.71.148 1.074.195.323.041.6-.218.6-.544v-4.661a6.714 6.714 0 0 \
                            1-.937-.171.75.75 0 1 1 .374-1.453 5.261 5.261 0 0 0 2.626 0 .75.75 0 1 1 .374 1.452 \
                            6.712 6.712 0 0 1-.937.172v4.66c0 .327.277.586.6.545.364-.047.722-.112 1.074-.195a.75.75 \
                            0 0 0 .577-.706c.02-.615.448-1.225 1.134-1.623A8.25 8.25 0 0 0 12 .75Z"}
                     />
                    <path
                        fillRule={"evenodd" }
                        d={"M9.013 19.9a.75.75 0 0 1 .877-.597 11.319 11.319 0 0 0 4.22 0 .75.75 0 1 1 .28 1.473 12.819 \
                            12.819 0 0 1-4.78 0 .75.75 0 0 1-.597-.876ZM9.754 22.344a.75.75 0 0 1 .824-.668 13.682 13.682 0 0 \
                            0 2.844 0 .75.75 0 1 1 .156 1.492 15.156 15.156 0 0 1-3.156 0 .75.75 0 0 1-.668-.824Z"}
                        clipRule={"evenodd" }
                    />
                </svg>
            },
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub icon: IconType,
}

#[function_component(Icon)]
pub fn icon_component(props: &IconProps) -> Html {
    props.icon.convert_to_svg()
}
