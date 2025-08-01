/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  darkMode: ["class"],
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },

    extend: {
      gridTemplateRows: {
        layout: "1fr auto",
      },
      fontFamily: {
        sans: [
          "Inter",
          "IBM Plex Mono",
          "GeistSans",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "Roboto",
          "Oxygen",
          "Ubuntu",
          "Cantarell",
          "Open Sans",
          "Helvetica Neue",
          "sans-serif",
        ],
        mono: [
          "IBM Plex Mono",
          "Menlo",
          "JetBrains Mono",
          "SFMono-Regular",
          "Monaco",
          "Consolas",
          "Liberation Mono",
          "Courier New",
          "monospace",
        ],
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        "secondary-background": "hsl(var(--secondary-background))",
        "tertiary-background": "hsl(var(--tertiary-background))",
        foreground: "hsl(var(--foreground))",
        "primary-text": "hsl(var(--primary-text))",
        "primary-text-muted": "hsl(var(--primary-text-muted))",
        "secondary-text": "hsl(var(--secondary-text))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        error: {
          DEFAULT: "hsl(var(--error))",
          foreground: "hsl(var(--error-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
          active: "hsl(var(--card-active))",
        },
        surreal: {
          5: "#fe1ca9",
          50: "#ffe8fb",
          100: "#ffcfef",
          200: "#ff9bda",
          300: "#ff64c5",
          400: "#fe38b4",
          500: "#fe1ca9",
          600: "#ff00a0",
          700: "#e4008e",
          800: "#cc007f",
          900: "#b3006f",
        },
        slate: {
          0: "#f1f1f3",
          1: "#d6d6dc",
          2: "#bbbbc4",
          3: "#9f9fac",
          4: "#848495",
          5: "#6a6a7b",
          6: "#39393c",
          7: "#2b2b2f",
          8: "#222226",
          9: "#19191D",
        },
      },
      borderRadius: {
        xl: "1rem",
        lg: "0.5rem",
        md: "0.375rem",
        sm: "0.125rem",
        xs: "0.1rem",
      },
      spacing: {
        xs: "1.5rem",
        sm: "2.25rem",
        md: "3rem",
        lg: "4rem",
        xl: "5rem",
      },
      // rgba(0,0,0,0.1) 1px,
      backgroundImage: {
        "grid-lines": `
          repeating-linear-gradient(
            0deg,
            rgba(0,0,0,0.1) 0,
            hsl(var(--primary-text-muted) / 0.2) 2px,
            transparent 1px,
            transparent 100%
          ),
          repeating-linear-gradient(
            90deg,
            rgba(0,0,0,0.1) 0,
            hsl(var(--primary-text-muted) / 0.2) 2px,
            transparent 1px,
            transparent 100%
          )
        `,
      },
      backgroundSize: {
        "grid-20": "20px 20px",
        "grid-30": "30px 30px",
        "grid-40": "40px 40px",
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
    },
  },
  plugins: [require("tailwindcss-animate"), require("@tailwindcss/typography")],
};
