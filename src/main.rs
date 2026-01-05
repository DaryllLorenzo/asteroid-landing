use yew::prelude::*;
use web_sys::window;

enum Msg {
    ToggleTheme,
    ScrollTo(&'static str),
}

struct LandingPage {
    dark_mode: bool,
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::ScrollTo(id) => {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(id) {
                            element.scroll_into_view_with_bool(true);
                        }
                    }
                }
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let theme_class = if self.dark_mode { "dark" } else { "light" };
        let theme_icon = if self.dark_mode { "🌙" } else { "☀️" };
        
        let on_scroll = |id: &'static str| Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(id) {
                        element.scroll_into_view_with_bool(true);
                    }
                }
            }
        });

        html! {
            <div class={format!("app-container {}", theme_class)}>
                <header class="header">
                    <div class="logo-container">
                        <img 
                            src="/assets/icon.png" 
                            alt="Asteroid Icon" 
                            class="app-icon"
                        />
                        <div class="logo">{"ASTEROID"}</div>
                    </div>
                    <nav class="nav">
                        <a href="#hero" onclick={on_scroll("hero")}>{"Inicio"}</a>
                        <a href="#features" onclick={on_scroll("features")}>{"Características"}</a>
                        <a href="#downloads" onclick={on_scroll("downloads")}>{"Descargas"}</a>
                        <button class="theme-toggle" onclick={_ctx.link().callback(|_| Msg::ToggleTheme)}>
                            {theme_icon}
                        </button>
                    </nav>
                </header>

                <main>
                    <section id="hero" class="hero">
                        <div class="hero-content">
                            <h1>{"Modelado Tropos, simplificado."}</h1>
                            <p>{"Asteroid lleva la poderosa metodología Tropos a tu escritorio. Una herramienta intuitiva para diseñar, analizar y visualizar sistemas complejos."}</p>
                        </div>
                    </section>

                    <section id="features" class="features">
                        <h2>{"¿Por qué Asteroid?"}</h2>
                        <div class="features-grid">
                            <div class="feature-card">
                                <div class="feature-icon">{"🎯"}</div>
                                <h3>{"Fiel a Tropos"}</h3>
                                <p>{"Implementa los conceptos clave de la metodología (actores, metas, dependencias)."}</p>
                            </div>
                            <div class="feature-card">
                                <div class="feature-icon">{"📦"}</div>
                                <h3>{"Portable"}</h3>
                                <p>{"Ejecutable en Windows y Linux sin instalación compleja. Fácil de desplegar y usar."}</p>
                            </div>
                            <div class="feature-card">
                                <div class="feature-icon">{"🖼️"}</div>
                                <h3>{"Exportable"}</h3>
                                <p>{"Guarda tus modelos en formato .astr o exporta como imágenes .png para compartir."}</p>
                            </div>
                        </div>
                    </section>

                    <section id="downloads" class="downloads">
                        <h2>{"Descarga Asteroid"}</h2>
                        <div class="download-options">
                            <a href="https://github.com/DaryllLorenzo/asteroid/releases/windows" class="download-card windows">
                                <div class="os-icon">{"🪟"}</div>
                                <div class="download-info">
                                    <h3>{"Windows"}</h3>
                                    <p>{"v1.0.0"}</p>
                                </div>
                                <div class="download-cta">{"Descargar"}</div>
                            </a>
                            <a href="https://github.com/DaryllLorenzo/asteroid/releases/linux" class="download-card linux">
                                <div class="os-icon">{"🐧"}</div>
                                <div class="download-info">
                                    <h3>{"Linux"}</h3>
                                    <p>{"v1.0.0"}</p>
                                </div>
                                <div class="download-cta">{"Descargar"}</div>
                            </a>
                        </div>
                        <p class="compatibility">{"Disponible para Windows 10/11 y distribuciones Linux de 64-bit."}</p>
                    </section>
                </main>

                <footer id="contact" class="footer">
                    <div class="footer-content">
                        <div class="copyright">
                            {"© 2025 Asteroid Project. Creado por Daryll Lorenzo."}
                        </div>
                        <div class="social-links">
                            <div class="github-stats">
                                <a href="https://github.com/DaryllLorenzo/asteroid" target="_blank" class="github-link" aria-label="GitHub Repository">
                                    <svg viewBox="0 0 24 24" width="20" height="20">
                                        <path fill="currentColor" d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                                    </svg>
                                    <span class="repo-name">{"asteroid"}</span>
                                </a>
                                <div class="stars">
                                    <span class="star-icon">{"⭐"}</span>
                                    <span class="star-count" id="github-stars">{"..."}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </footer>
            </div>
        }
    }
}


fn main() {
    yew::Renderer::<LandingPage>::new().render();
}