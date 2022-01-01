//! Providing icons for different filetypes.

use std::collections::HashMap;

pub fn get_icons_by_extension() -> HashMap<&'static str, &'static str> {
    let mut icon_map = HashMap::new();

    icon_map.insert("7z", "\u{f410}"); // ""
    icon_map.insert("apk", "\u{e70e}"); // ""
    icon_map.insert("avi", "\u{f03d}"); // ""
    icon_map.insert("avro", "\u{e60b}"); // ""
    icon_map.insert("awk", "\u{f489}"); // ""
    icon_map.insert("bash", "\u{f489}"); // ""
    icon_map.insert("bash_history", "\u{f489}"); // ""
    icon_map.insert("bash_profile", "\u{f489}"); // ""
    icon_map.insert("bashrc", "\u{f489}"); // ""
    icon_map.insert("bat", "\u{f17a}"); // ""
    icon_map.insert("bio", "\u{f910}"); // "蘿"
    icon_map.insert("bmp", "\u{f1c5}"); // ""
    icon_map.insert("bz2", "\u{f410}"); // ""
    icon_map.insert("c", "\u{e61e}"); // ""
    icon_map.insert("c++", "\u{e61d}"); // ""
    icon_map.insert("cc", "\u{e61d}"); // ""
    icon_map.insert("cfg", "\u{e615}"); // ""
    icon_map.insert("clj", "\u{e768}"); // ""
    icon_map.insert("cljs", "\u{e76a}"); // ""
    icon_map.insert("cls", "\u{e600}"); // ""
    icon_map.insert("coffee", "\u{f0f4}"); // ""
    icon_map.insert("conf", "\u{e615}"); // ""
    icon_map.insert("cp", "\u{e61d}"); // ""
    icon_map.insert("cpp", "\u{e61d}"); // ""
    icon_map.insert("cs", "\u{f81a}"); // ""
    icon_map.insert("cshtml", "\u{f1fa}"); // ""
    icon_map.insert("csproj", "\u{f81a}"); // ""
    icon_map.insert("csx", "\u{f81a}"); // ""
    icon_map.insert("csh", "\u{f489}"); // ""
    icon_map.insert("css", "\u{e749}"); // ""
    icon_map.insert("csv", "\u{f1c3}"); // ""
    icon_map.insert("cxx", "\u{e61d}"); // ""
    icon_map.insert("d", "\u{e7af}"); // ""
    icon_map.insert("dart", "\u{e798}"); // ""
    icon_map.insert("db", "\u{f1c0}"); // ""
    icon_map.insert("diff", "\u{f440}"); // ""
    icon_map.insert("doc", "\u{f1c2}"); // ""
    icon_map.insert("docx", "\u{f1c2}"); // ""
    icon_map.insert("ds_store", "\u{f179}"); // ""
    icon_map.insert("dump", "\u{f1c0}"); // ""
    icon_map.insert("ebook", "\u{e28b}"); // ""
    icon_map.insert("editorconfig", "\u{e615}"); // ""
    icon_map.insert("ejs", "\u{e618}"); // ""
    icon_map.insert("elm", "\u{e62c}"); // ""
    icon_map.insert("env", "\u{f462}"); // ""
    icon_map.insert("eot", "\u{f031}"); // ""
    icon_map.insert("epub", "\u{e28a}"); // ""
    icon_map.insert("erb", "\u{e73b}"); // ""
    icon_map.insert("erl", "\u{e7b1}"); // ""
    icon_map.insert("exe", "\u{f17a}"); // ""
    icon_map.insert("ex", "\u{e62d}"); // ""
    icon_map.insert("exs", "\u{e62d}"); // ""
    icon_map.insert("fish", "\u{f489}"); // ""
    icon_map.insert("flac", "\u{f001}"); // ""
    icon_map.insert("flv", "\u{f03d}"); // ""
    icon_map.insert("font", "\u{f031}"); // ""
    icon_map.insert("fpl", "\u{f910}"); // "蘿"
    icon_map.insert("gdoc", "\u{f1c2}"); // ""
    icon_map.insert("gemfile", "\u{e21e}"); // ""
    icon_map.insert("gemspec", "\u{e21e}"); // ""
    icon_map.insert("gform", "\u{f298}"); // ""
    icon_map.insert("gif", "\u{f1c5}"); // ""
    icon_map.insert("git", "\u{f1d3}"); // ""
    icon_map.insert("go", "\u{e626}"); // ""
    icon_map.insert("gradle", "\u{e70e}"); // ""
    icon_map.insert("gsheet", "\u{f1c3}"); // ""
    icon_map.insert("gslides", "\u{f1c4}"); // ""
    icon_map.insert("guardfile", "\u{e21e}"); // ""
    icon_map.insert("gz", "\u{f410}"); // ""
    icon_map.insert("h", "\u{f0fd}"); // ""
    icon_map.insert("hbs", "\u{e60f}"); // ""
    icon_map.insert("hpp", "\u{f0fd}"); // ""
    icon_map.insert("hs", "\u{e777}"); // ""
    icon_map.insert("htm", "\u{f13b}"); // ""
    icon_map.insert("html", "\u{f13b}"); // ""
    icon_map.insert("hxx", "\u{f0fd}"); // ""
    icon_map.insert("ico", "\u{f1c5}"); // ""
    icon_map.insert("image", "\u{f1c5}"); // ""
    icon_map.insert("iml", "\u{e7b5}"); // ""
    icon_map.insert("ini", "\u{e615}"); // ""
    icon_map.insert("ipynb", "\u{e606}"); // ""
    icon_map.insert("jar", "\u{e204}"); // ""
    icon_map.insert("java", "\u{e204}"); // ""
    icon_map.insert("jpeg", "\u{f1c5}"); // ""
    icon_map.insert("jpg", "\u{f1c5}"); // ""
    icon_map.insert("js", "\u{e74e}"); // ""
    icon_map.insert("json", "\u{e60b}"); // ""
    icon_map.insert("jsx", "\u{e7ba}"); // ""
    icon_map.insert("jl", "\u{e624}"); // ""
    icon_map.insert("ksh", "\u{f489}"); // ""
    icon_map.insert("less", "\u{e758}"); // ""
    icon_map.insert("lhs", "\u{e777}"); // ""
    icon_map.insert("license", "\u{f48a}"); // ""
    icon_map.insert("localized", "\u{f179}"); // ""
    icon_map.insert("lock", "\u{f023}"); // ""
    icon_map.insert("log", "\u{f18d}"); // ""
    icon_map.insert("lua", "\u{e620}"); // ""
    icon_map.insert("lz", "\u{f410}"); // ""
    icon_map.insert("m3u", "\u{f910}"); // "蘿"
    icon_map.insert("m3u8", "\u{f910}"); // "蘿"
    icon_map.insert("m4a", "\u{f001}"); // ""
    icon_map.insert("markdown", "\u{f48a}"); // ""
    icon_map.insert("md", "\u{f48a}"); // ""
    icon_map.insert("mkd", "\u{f48a}"); // ""
    icon_map.insert("mkv", "\u{f03d}"); // ""
    icon_map.insert("mobi", "\u{e28b}"); // ""
    icon_map.insert("mov", "\u{f03d}"); // ""
    icon_map.insert("mp3", "\u{f001}"); // ""
    icon_map.insert("mp4", "\u{f03d}"); // ""
    icon_map.insert("mustache", "\u{e60f}"); // ""
    icon_map.insert("nix", "\u{f313}"); // ""
    icon_map.insert("npmignore", "\u{e71e}"); // ""
    icon_map.insert("opus", "\u{f001}"); // ""
    icon_map.insert("ogg", "\u{f001}"); // ""
    icon_map.insert("ogv", "\u{f03d}"); // ""
    icon_map.insert("otf", "\u{f031}"); // ""
    icon_map.insert("pdf", "\u{f1c1}"); // ""
    icon_map.insert("php", "\u{e73d}"); // ""
    icon_map.insert("pl", "\u{e769}"); // ""
    icon_map.insert("pls", "\u{f910}"); // "蘿"
    icon_map.insert("png", "\u{f1c5}"); // ""
    icon_map.insert("ppt", "\u{f1c4}"); // ""
    icon_map.insert("pptx", "\u{f1c4}"); // ""
    icon_map.insert("procfile", "\u{e21e}"); // ""
    icon_map.insert("properties", "\u{e60b}"); // ""
    icon_map.insert("ps1", "\u{f489}"); // ""
    icon_map.insert("psd", "\u{e7b8}"); // ""
    icon_map.insert("pxm", "\u{f1c5}"); // ""
    icon_map.insert("py", "\u{e606}"); // ""
    icon_map.insert("pyc", "\u{e606}"); // ""
    icon_map.insert("r", "\u{f25d}"); // ""
    icon_map.insert("rakefile", "\u{e21e}"); // ""
    icon_map.insert("rar", "\u{f410}"); // ""
    icon_map.insert("razor", "\u{f1fa}"); // ""
    icon_map.insert("rb", "\u{e21e}"); // ""
    icon_map.insert("rdata", "\u{f25d}"); // ""
    icon_map.insert("rdb", "\u{e76d}"); // ""
    icon_map.insert("rdoc", "\u{f48a}"); // ""
    icon_map.insert("rds", "\u{f25d}"); // ""
    icon_map.insert("readme", "\u{f48a}"); // ""
    icon_map.insert("rlib", "\u{e7a8}"); // ""
    icon_map.insert("rmd", "\u{f48a}"); // ""
    icon_map.insert("rs", "\u{e7a8}"); // ""
    icon_map.insert("rspec", "\u{e21e}"); // ""
    icon_map.insert("rspec_parallel", "\u{e21e}"); // ""
    icon_map.insert("rspec_status", "\u{e21e}"); // ""
    icon_map.insert("rss", "\u{f09e}"); // ""
    icon_map.insert("ru", "\u{e21e}"); // ""
    icon_map.insert("rubydoc", "\u{e73b}"); // ""
    icon_map.insert("sass", "\u{e603}"); // ""
    icon_map.insert("scala", "\u{e737}"); // ""
    icon_map.insert("scss", "\u{e749}"); // ""
    icon_map.insert("sh", "\u{f489}"); // ""
    icon_map.insert("shell", "\u{f489}"); // ""
    icon_map.insert("slim", "\u{e73b}"); // ""
    icon_map.insert("sln", "\u{e70c}"); // ""
    icon_map.insert("sql", "\u{f1c0}"); // ""
    icon_map.insert("sqlite3", "\u{e7c4}"); // ""
    icon_map.insert("styl", "\u{e600}"); // ""
    icon_map.insert("stylus", "\u{e600}"); // ""
    icon_map.insert("svg", "\u{f1c5}"); // ""
    icon_map.insert("swift", "\u{e755}"); // ""
    icon_map.insert("tar", "\u{f410}"); // ""
    icon_map.insert("tex", "\u{e600}"); // ""
    icon_map.insert("tiff", "\u{f1c5}"); // ""
    icon_map.insert("ts", "\u{e628}"); // ""
    icon_map.insert("tsx", "\u{e7ba}"); // ""
    icon_map.insert("ttc", "\u{f031}"); // ""
    icon_map.insert("ttf", "\u{f031}"); // ""
    icon_map.insert("twig", "\u{e61c}"); // ""
    icon_map.insert("txt", "\u{f15c}"); // ""
    icon_map.insert("video", "\u{f03d}"); // ""
    icon_map.insert("vim", "\u{e62b}"); // ""
    icon_map.insert("vlc", "\u{f910}"); // "蘿"
    icon_map.insert("vue", "\u{fd42}"); // "﵂"
    icon_map.insert("wav", "\u{f001}"); // ""
    icon_map.insert("webm", "\u{f03d}"); // ""
    icon_map.insert("webp", "\u{f1c5}"); // ""
    icon_map.insert("windows", "\u{f17a}"); // ""
    icon_map.insert("wma", "\u{f001}"); // ""
    icon_map.insert("wmv", "\u{f03d}"); // ""
    icon_map.insert("wpl", "\u{f910}"); // "蘿"
    icon_map.insert("woff", "\u{f031}"); // ""
    icon_map.insert("woff2", "\u{f031}"); // ""
    icon_map.insert("xls", "\u{f1c3}"); // ""
    icon_map.insert("xlsx", "\u{f1c3}"); // ""
    icon_map.insert("xml", "\u{e619}"); // ""
    icon_map.insert("xul", "\u{e619}"); // ""
    icon_map.insert("xz", "\u{f410}"); // ""
    icon_map.insert("yaml", "\u{e60b}"); // ""
    icon_map.insert("yml", "\u{e60b}"); // ""
    icon_map.insert("zip", "\u{f410}"); // ""
    icon_map.insert("zsh", "\u{f489}"); // ""
    icon_map.insert("zsh-theme", "\u{f489}"); // ""
    icon_map.insert("zshrc", "\u{f489}"); // ""

    icon_map
}

pub fn get_icons_by_name() -> HashMap<&'static str, &'static str> {
    let mut icon_map = HashMap::new();

    icon_map.insert(".trash", "\u{f1f8}"); // ""
    icon_map.insert(".atom", "\u{e764}"); // ""
    icon_map.insert(".bashprofile", "\u{e615}"); // ""
    icon_map.insert(".bashrc", "\u{f489}"); // ""
    icon_map.insert(".git", "\u{f1d3}"); // ""
    icon_map.insert(".gitattributes", "\u{f1d3}"); // ""
    icon_map.insert(".gitconfig", "\u{f1d3}"); // ""
    icon_map.insert(".github", "\u{f408}"); // ""
    icon_map.insert(".gitignore", "\u{f1d3}"); // ""
    icon_map.insert(".gitmodules", "\u{f1d3}"); // ""
    icon_map.insert(".rvm", "\u{e21e}"); // ""
    icon_map.insert(".vimrc", "\u{e62b}"); // ""
    icon_map.insert(".vscode", "\u{e70c}"); // ""
    icon_map.insert(".zshrc", "\u{f489}"); // ""
    icon_map.insert("bin", "\u{e5fc}"); // ""
    icon_map.insert("config", "\u{e5fc}"); // ""
    icon_map.insert("docker-compose.yml", "\u{f308}"); // ""
    icon_map.insert("dockerfile", "\u{f308}"); // ""
    icon_map.insert("ds_store", "\u{f179}"); // ""
    icon_map.insert("gitignore_global", "\u{f1d3}"); // ""
    icon_map.insert("gradle", "\u{e70e}"); // ""
    icon_map.insert("gruntfile.coffee", "\u{e611}"); // ""
    icon_map.insert("gruntfile.js", "\u{e611}"); // ""
    icon_map.insert("gruntfile.ls", "\u{e611}"); // ""
    icon_map.insert("gulpfile.coffee", "\u{e610}"); // ""
    icon_map.insert("gulpfile.js", "\u{e610}"); // ""
    icon_map.insert("gulpfile.ls", "\u{e610}"); // ""
    icon_map.insert("hidden", "\u{f023}"); // ""
    icon_map.insert("include", "\u{e5fc}"); // ""
    icon_map.insert("lib", "\u{f121}"); // ""
    icon_map.insert("localized", "\u{f179}"); // ""
    icon_map.insert("node_modules", "\u{e718}"); // ""
    icon_map.insert("npmignore", "\u{e71e}"); // ""
    icon_map.insert("rubydoc", "\u{e73b}"); // ""

    icon_map
}

#[cfg(test)]
mod test_icons {
    use super::*;

    #[test]
    fn test_get_icons_by_extension_valid_key() {
        let icons = get_icons_by_extension();

        assert_eq!(icons.get("rs"), Some(&"\u{e7a8}"));
    }

    #[test]
    fn test_get_icons_by_extension_invalid_key() {
        let icons = get_icons_by_extension();

        assert_eq!(icons.get("asdf"), None);
    }

    #[test]
    fn test_get_icons_by_name_valid_key() {
        let icons = get_icons_by_name();

        assert_eq!(icons.get(".vimrc"), Some(&"\u{e62b}"));
    }

    #[test]
    fn test_get_icons_by_name_invalid_key() {
        let icons = get_icons_by_name();

        assert_eq!(icons.get("asdf"), None);
    }
}
