use yew::prelude::*;
use web_sys::window;
use js_sys;

enum Msg {
    ToggleTheme,
    ToggleMenu,
    ScrollTo(&'static str),
}

struct LandingPage {
    dark_mode: bool,
    menu_open: bool,
}

impl Component for LandingPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let prefers_dark = window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map_or(false, |mq| mq.matches());

        Self {
            dark_mode: prefers_dark,
            menu_open: false,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let _ = js_sys::eval(r#"
                (function() {
                    var observer = new IntersectionObserver(function(entries) {
                        entries.forEach(function(e) {
                            if (e.isIntersecting) {
                                e.target.classList.add('visible');
                                observer.unobserve(e.target);
                            }
                        });
                    }, { threshold: 0.12 });
                    document.querySelectorAll('.fade-up').forEach(function(el) {
                        observer.observe(el);
                    });
                })();
            "#);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::ToggleMenu => {
                self.menu_open = !self.menu_open;
                true
            }
            Msg::ScrollTo(id) => {
                self.menu_open = false;
                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.get_element_by_id(id) {
                            el.scroll_into_view_with_bool(true);
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme_class = if self.dark_mode { "dark" } else { "light" };
        let theme_icon = if self.dark_mode { "☀️" } else { "🌙" };
        let menu_class = if self.menu_open { "mobile-menu open" } else { "mobile-menu" };
        let hamburger_class = if self.menu_open { "hamburger active" } else { "hamburger" };

        let scroll = |id: &'static str| {
            ctx.link().callback(move |e: MouseEvent| {
                e.prevent_default();
                Msg::ScrollTo(id)
            })
        };

        html! {
            <div class={format!("app {}", theme_class)}>

                <header class="header">
                    <div class="header-inner">
                        <a href="#hero" class="logo" onclick={scroll("hero")}>
                            <img src="assets/icon.png" alt="" class="logo-icon" />
                            {"ASTEROID"}
                        </a>

                        // Desktop nav
                        <nav class="nav-desktop">
                            <a href="#hero"      onclick={scroll("hero")}>{"Inicio"}</a>
                            <a href="#features"  onclick={scroll("features")}>{"Características"}</a>
                            <a href="#demo"      onclick={scroll("demo")}>{"Demo"}</a>
                            <a href="#downloads" onclick={scroll("downloads")}>{"Descargas"}</a>
                            <button
                                class="theme-toggle"
                                onclick={ctx.link().callback(|_| Msg::ToggleTheme)}
                                aria-label="Cambiar tema"
                            >
                                {theme_icon}
                            </button>
                        </nav>

                        // Mobile controls
                        <div class="nav-mobile-controls">
                            <button
                                class="theme-toggle"
                                onclick={ctx.link().callback(|_| Msg::ToggleTheme)}
                                aria-label="Cambiar tema"
                            >
                                {theme_icon}
                            </button>
                            <button
                                class={hamburger_class}
                                onclick={ctx.link().callback(|_| Msg::ToggleMenu)}
                                aria-label="Menú"
                            >
                                <span></span>
                                <span></span>
                                <span></span>
                            </button>
                        </div>
                    </div>
                </header>

                <div class={menu_class}>
                    <nav class="mobile-nav">
                        <a href="#hero"      onclick={scroll("hero")}>{"Inicio"}</a>
                        <a href="#features"  onclick={scroll("features")}>{"Características"}</a>
                        <a href="#downloads" onclick={scroll("downloads")}>{"Descargas"}</a>
                    </nav>
                </div>

                <main>
                    // Hero
                    <section id="hero" class="hero">
                        <div class="hero-inner">
                            <p class="hero-eyebrow fade-up" style="transition-delay: 0ms;">
                                {"Modelado Tropos"}
                            </p>
                            <h1 class="fade-up" style="transition-delay: 80ms;">
                                {"Diseña sistemas"}
                                <br/>
                                {"complejos con claridad."}
                            </h1>
                            <p class="hero-sub fade-up" style="transition-delay: 160ms;">
                                {"Asteroid lleva la metodología Tropos a tu escritorio. Actores, metas y dependencias en una herramienta intuitiva y portable."}
                            </p>
                            <a href="#downloads" class="cta-btn fade-up" style="transition-delay: 240ms;" onclick={scroll("downloads")}>
                                {"Descargar gratis"}
                            </a>
                        </div>
                    </section>

                    // Features
                    <section id="features" class="features">
                        <div class="section-inner">
                            <h2 class="fade-up" style="transition-delay: 0ms;">{"¿Por qué Asteroid?"}</h2>
                            <div class="features-grid">
                                <div class="feature-card fade-up" style="transition-delay: 0ms;">
                                    <div class="feature-icon">{"🎯"}</div>
                                    <h3>{"Fiel a Tropos"}</h3>
                                    <p>{"Implementa actores, metas y dependencias según la metodología, sin atajos."}</p>
                                </div>
                                <div class="feature-card fade-up" style="transition-delay: 100ms;">
                                    <div class="feature-icon">{"📦"}</div>
                                    <h3>{"Portable"}</h3>
                                    <p>{"Binario único para Windows, Linux y macOS. Sin instaladores ni dependencias."}</p>
                                </div>
                                <div class="feature-card fade-up" style="transition-delay: 200ms;">
                                    <div class="feature-icon">{"🖼️"}</div>
                                    <h3>{"Exportable"}</h3>
                                    <p>{"Guarda en formato .astr o exporta como .png para presentar tu modelo."}</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    // Demo
                    <section id="demo" class="demo">
                        <div class="section-inner">
                            <h2 class="fade-up" style="transition-delay: 0ms;">
                                {"Mira Asteroid en acción"}
                            </h2>
                            <p class="section-sub fade-up" style="transition-delay: 60ms;">
                                {"Demos rápidas de las funcionalidades principales"}
                            </p>

                            <div class="demo-grid">
                                <div class="demo-card fade-up" style="transition-delay: 0ms;">
                                    <video class="demo-video" autoplay=true loop=true muted=true playsinline=true>
                                        <source src="assets/videos/video1.mp4" type="video/mp4" />
                                    </video>
                                    <h3>{"Actores"}</h3>
                                    <p>{"Canvas de comportamiento de los actores y sus metas."}</p>
                                </div>

                                <div class="demo-card fade-up" style="transition-delay: 100ms;">
                                    <video class="demo-video" autoplay=true loop=true muted=true playsinline=true>
                                        <source src="assets/videos/video2.mp4" type="video/mp4" />
                                    </video>
                                    <h3>{"Relaciones de comportamiento"}</h3>
                                    <p>{"Enlaces entre elementos de comportamiento: metas, planes, recursos y links de Tropos."}</p>
                                </div>

                                <div class="demo-card fade-up" style="transition-delay: 200ms;">
                                    <video class="demo-video" autoplay=true loop=true muted=true playsinline=true>
                                        <source src="assets/videos/video3.mp4" type="video/mp4" />
                                    </video>
                                    <h3>{"Dependencias entre actores"}</h3>
                                    <p>{"Relaciones de dependencia entre actores y agentes."}</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    // Downloads
                    <section id="downloads" class="downloads">
                        <div class="section-inner">
                            <h2 class="fade-up" style="transition-delay: 0ms;">{"Descarga Asteroid"}</h2>
                            <p class="section-sub fade-up" style="transition-delay: 60ms;">
                                {"v0.2.0 — Windows, Linux y macOS"}
                            </p>
                            <div class="download-grid">
                                <a href="https://github.com/DaryllLorenzo/Asteroid/releases/download/v0.2.0/asteroid_0.2.0.exe"
                                   class="download-card fade-up"
                                   style="transition-delay: 0ms;">
                                    <img src="assets/windows11-original.svg" alt="Windows" class="os-icon" />
                                    <div class="download-info">
                                        <h3>{"Windows"}</h3>
                                        <p>{"Windows 10 / 11"}</p>
                                    </div>
                                    <span class="dl-arrow">{"↓"}</span>
                                </a>
                                <a href="https://github.com/DaryllLorenzo/Asteroid/releases/download/v0.2.0/asteroid_0.2.0_amd64.deb"
                                   class="download-card fade-up"
                                   style="transition-delay: 80ms;">
                                    <img src="assets/linux-original.svg" alt="Linux" class="os-icon" />
                                    <div class="download-info">
                                        <h3>{"Linux"}</h3>
                                        <p>{"64-bit .deb"}</p>
                                    </div>
                                    <span class="dl-arrow">{"↓"}</span>
                                </a>
                                <a href="https://github.com/DaryllLorenzo/Asteroid/releases/download/v0.2.0/asteroid_0.2.0.app.zip"
                                   class="download-card fade-up"
                                   style="transition-delay: 160ms;">
                                    <img src="assets/apple-original.svg" alt="macOS" class="os-icon" />
                                    <div class="download-info">
                                        <h3>{"macOS"}</h3>
                                        <p>{"App bundle .zip"}</p>
                                    </div>
                                    <span class="dl-arrow">{"↓"}</span>
                                </a>
                            </div>
                        </div>
                    </section>
                </main>

                <footer class="footer">
                    <div class="footer-inner">
                        <span class="footer-copy">{"© 2025 Asteroid Project · Daryll Lorenzo"}</span>
                        <div class="footer-right">
                            <a href="https://github.com/DaryllLorenzo/asteroid"
                               target="_blank"
                               class="github-link"
                               aria-label="GitHub">
                                <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
                                    <path fill="currentColor" d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                                </svg>
                                {"asteroid"}
                            </a>
                            <span class="github-stars">
                                {"⭐ "}
                                <span id="github-stars">{"·"}</span>
                            </span>
                        </div>
                    </div>
                </footer>
            </div>
        }
    }
}

fn main() {
    let element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap();
    yew::Renderer::<LandingPage>::with_root(element).render();
}