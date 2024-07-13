import { Config } from "tailwindcss";
import typography from "@tailwindcss/typography";

const config: Config = {
    content: ["./src/**/*.{html,rs}"],
    darkMode: "class",
    theme: {
        extend: {
            fontFamily: {
                "open-dyslexic": ["Open Dyslexic"],
            },
            boxShadow: {
                "light-blahaj-regular": "10px 10px 0px #F2BED1", // light-blahaj-400
                "light-blahaj-depress": "6px 6px 0px #F2BED1", // same color, -0.25em -0.25em
                "light-blahaj-depress-horizontal": "6px 10px 0px #F2BED1", // same color, -0.25em -0
                "dark-blahaj-regular": "10px 10px 0px #4C0033", // dark-blåhaj-300
                "dark-blahaj-depress": "6px 6px 0px #4C0033", // same color, -0.25em -0.25em
                "dark-blahaj-depress-horizontal": "6px 10px 0px #4C0033", // same color, -0.25em -0
            },
            width: {
                "5xl": "64rem",
            },
            colors: {
                light: {
                    blåhaj: {
                        "100": "hsl(345, 25, 97)", // unused
                        "200": "hsl(338, 52, 96)",
                        "300": "hsl(338, 53, 94)",
                        "400": "hsl(337, 100, 92)",
                        "500": "hsl(338, 92, 90)",
                        "600": "hsl(338, 67, 85)",
                        "700": "hsl(335, 36, 79)",
                    },
                },
                dark: {
                    blåhaj: {
                        "100": "#3B414D",
                        "200": "#2E343E",
                        "300": "#3A3939",
                        "400": "#333333",
                        "500": "#272727",
                        "600": "#1E1E1E",
                    },
                },
                primary: {
                    pink: "hsl(330, 100, 71)",
                    peach: "hsl(26, 76, 77)",
                    dark: "#AAA498",
                },
            },
        },
    },
    plugins: [typography],
};

export default config;
