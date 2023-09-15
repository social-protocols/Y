window.yDarkMode = false;

if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    window.yDarkMode = true;
    console.log("Dark mode is: " + window.yDarkMode);
}
