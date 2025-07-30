/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    // The best breakpoints: https://medium.com/free-code-camp/the-100-correct-way-to-do-css-breakpoints-88d6a5ba1862
    // Optionally introduce mobile breakpoint at 300px and then replace the body with this html:
    // 
    // <body class="hidden mobile:flex flex-col h-screen"></body>
    // <body class="block mobile:hidden">
    //    <p>Sorry, we this websites doesn't work on toasters/fridges/watches or other screens with a width of under 300px.</p>
    // </body>
    // 
    // ;)
    // 
    screens: {
      tablet: "600px",
      notebook: "900px",
      desktop: "1200px",
      tv: "1800px",
    },
  },
  plugins: [],
}
