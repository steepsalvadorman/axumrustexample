use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="nav-minimal fixed top-0 w-full z-50 px-8 py-5 flex justify-between items-center transition-all duration-300">
                <div class="flex items-center gap-3">
                    <span class="text-2xl font-serif font-black tracking-tight text-warm-900">"PetHaven"</span>
                </div>
                <div class="hidden md:flex items-center gap-10 text-xs font-semibold uppercase tracking-widest text-warm-600">
                    <a href="/" class="btn-elegant hover:text-warm-900 transition-colors">"Historias"</a>
                    <a href="#catalogo" class="btn-elegant hover:text-warm-900 transition-colors">"Venta"</a>
                    <a href="#adopcion" class="btn-elegant hover:text-warm-900 transition-colors">"Adopción"</a>
                </div>
                <div class="flex items-center gap-6">
                    <a href="http://localhost:3000/auth/keycloak-login" class="text-xs font-bold uppercase tracking-widest text-warm-900 border-b-2 border-sage-500 pb-1 hover:border-warm-900 transition-all">
                        "Mi Cuenta"
                    </a>
                </div>
            </nav>

            <main class="min-h-screen bg-warm-50">
                <Routes fallback=|| view! { <div class="p-40 text-center font-serif text-2xl">"Página no encontrada"</div> }>
                    <Route path=path!("") view=Home/>
                    <Route path=path!("dashboard") view=Dashboard/>
                </Routes>
            </main>

            <footer class="bg-warm-100 border-t border-warm-200 py-20 px-8">
                <div class="max-w-6xl mx-auto flex flex-col md:flex-row justify-between items-start gap-12">
                    <div class="space-y-6 max-w-sm">
                        <span class="text-3xl font-serif font-black text-warm-900">"PetHaven"</span>
                        <p class="text-warm-500 leading-relaxed italic">"Encontrando familias para perritos increíbles desde 2024."</p>
                    </div>
                    <div class="grid grid-cols-2 gap-16">
                        <div class="space-y-4">
                            <h4 class="text-[10px] font-black uppercase tracking-widest text-warm-400">"Contacto"</h4>
                            <p class="text-sm text-warm-700">"ladridos@pethaven.com"</p>
                        </div>
                        <div class="space-y-4">
                            <h4 class="text-[10px] font-black uppercase tracking-widest text-warm-400">"Ubicación"</h4>
                            <p class="text-sm text-warm-700">"Madrid, España"</p>
                        </div>
                    </div>
                </div>
            </footer>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (current_slide, set_current_slide) = signal(0);
    
    let images = vec![
        "/assets/dog1.png",
        "/assets/dog2.png",
        "/assets/dog3.png",
    ];

    let next_slide = move |_| {
        set_current_slide.update(|n| *n = (*n + 1) % 3);
    };

    let prev_slide = move |_| {
        set_current_slide.update(|n| *n = if *n == 0 { 2 } else { *n - 1 });
    };

    view! {
        <div class="space-y-24 pb-32">
            // Hero Carousel Section
            <section class="relative h-[85vh] w-full overflow-hidden bg-warm-200">
                <div class="absolute inset-0 z-0">
                    {move || view! {
                        <img 
                            src={images[current_slide.get()]} 
                            class="w-full h-full object-cover carousel-slide transition-all duration-1000"
                            alt="Cachorro del momento"
                        />
                    }}
                    <div class="absolute inset-0 bg-black/5"></div>
                </div>

                <div class="absolute inset-0 z-10 flex flex-col justify-center items-center text-center p-6">
                    <div class="max-w-4xl space-y-8 animate-fade-in text-white drop-shadow-lg">
                        <h1 class="text-6xl md:text-8xl font-serif font-black leading-tight">
                            "Amor en cuatro " <br/> "patitas."
                        </h1>
                        <p class="text-lg md:text-xl font-medium tracking-wide">
                            "Boutique de venta ética y refugio de adopción responsable."
                        </p>
                    </div>
                </div>

                <div class="absolute bottom-10 left-0 w-full z-20 flex justify-center items-center gap-12 text-white">
                    <button on:click=prev_slide class="hover:scale-125 transition-transform p-4">
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
                    </button>
                    <div class="flex gap-4">
                        {move || (0..3).map(|i| {
                            let active = i == current_slide.get();
                            view! { <div class=format!("h-1 transition-all duration-500 {}", if active { "w-12 bg-white" } else { "w-4 bg-white/40" })></div> }
                        }).collect_view()}
                    </div>
                    <button on:click=next_slide class="hover:scale-125 transition-transform p-4">
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
                    </button>
                </div>
            </section>

            // Stories Section (Adopción)
            <section class="max-w-6xl mx-auto px-8 space-y-10">
                <div class="flex justify-between items-end">
                    <div class="space-y-2">
                        <h2 class="text-3xl font-serif font-bold text-warm-900">"Historias de Adopción"</h2>
                        <p class="text-warm-500 italic text-sm">"Conoce a los que buscan un hogar"</p>
                    </div>
                    <a href="#" class="text-xs font-bold uppercase tracking-widest text-sage-600 border-b border-sage-500 pb-1">"Ver todas"</a>
                </div>

                <div class="flex overflow-x-auto gap-10 pb-6 no-scrollbar">
                    <StoryProfile name="Boby" img="/assets/dog1.png" />
                    <StoryProfile name="Lucas" img="/assets/dog2.png" />
                    <StoryProfile name="Coco" img="/assets/dog3.png" />
                    <StoryProfile name="Simba" img="/assets/dog4.png" />
                    <StoryProfile name="Max" img="/assets/dog1.png" />
                    <StoryProfile name="Nina" img="/assets/dog2.png" />
                </div>
            </section>

            // Catalog Section (Venta)
            <section id="catalogo" class="max-w-6xl mx-auto px-8 space-y-16 py-12">
                <div class="text-center space-y-4">
                    <h2 class="text-4xl font-serif font-bold text-warm-900">"Cachorros en Venta"</h2>
                    <p class="text-warm-500 italic">"Seleccionados por su salud y temperamento."</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-12">
                    <PetCard name="Labradoodle" price="1.450€" age="2 meses" category="Venta" img="/assets/dog1.png" />
                    <PetCard name="French Bulldog" price="1.800€" age="3 meses" category="Venta" img="/assets/dog2.png" />
                    <PetCard name="Golden Retriever" price="1.200€" age="2.5 meses" category="Venta" img="/assets/dog3.png" />
                </div>
            </section>

            // Adoption Section (Cards)
            <section id="adopcion" class="bg-warm-100 py-24">
                <div class="max-w-6xl mx-auto px-8 space-y-16">
                    <div class="text-center space-y-4">
                        <h2 class="text-4xl font-serif font-bold text-warm-900">"Buscan una Familia"</h2>
                        <p class="text-warm-500 italic">"Adoptar es un acto de amor incondicional."</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-12">
                        <PetCard name="Toby" price="Gratis" age="1 año" category="Adopción" img="/assets/dog4.png" />
                        <PetCard name="Mora" price="Gratis" age="6 meses" category="Adopción" img="/assets/dog1.png" />
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn StoryProfile(name: &'static str, img: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3 cursor-pointer group flex-shrink-0">
            <div class="w-24 h-24 rounded-full p-1 border-2 border-sage-500 group-hover:border-warm-900 transition-all duration-500">
                <img src={img} class="w-full h-full rounded-full object-cover transition-transform group-hover:scale-105" />
            </div>
            <span class="text-xs font-bold uppercase tracking-widest text-warm-600">{name}</span>
        </div>
    }
}

#[component]
fn PetCard(name: &'static str, price: &'static str, age: &'static str, category: &'static str, img: &'static str) -> impl IntoView {
    view! {
        <div class="group cursor-pointer space-y-6">
            <div class="img-hover aspect-square soft-shadow bg-warm-200 relative">
                 <img src={img} class="w-full h-full object-cover" />
                 <div class="absolute top-4 left-4">
                    <span class=format!("px-3 py-1 text-[9px] font-black uppercase tracking-tighter rounded-full text-white {}", 
                        if category == "Venta" { "bg-warm-900" } else { "bg-sage-600 shadow-lg shadow-sage-500/30" })>
                        {category}
                    </span>
                 </div>
            </div>
            <div class="flex justify-between items-end">
                <div>
                    <h3 class="text-xl font-serif font-bold italic text-warm-900 group-hover:text-sage-600 transition-colors">{name}</h3>
                    <p class="text-warm-400 text-[10px] font-bold uppercase tracking-widest">{age} " de edad"</p>
                </div>
                <span class="text-lg font-black text-warm-900">{price}</span>
            </div>
        </div>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div class="max-w-5xl mx-auto px-8 py-32 space-y-20">
            <header class="border-b border-warm-200 pb-12 text-center md:text-left">
                <h1 class="text-5xl font-serif font-bold text-warm-900">"Tu Perfil"</h1>
                <p class="text-warm-500 italic mt-2">"Gestiona tus adopciones y compras seguras."</p>
            </header>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-16">
                <div class="md:col-span-2 space-y-12">
                    <div class="bg-white soft-shadow rounded-3xl p-10 border border-warm-100 flex items-center justify-between">
                         <div class="flex items-center gap-8">
                            <div class="w-20 h-20 bg-warm-50 rounded-full border border-warm-100 flex items-center justify-center">
                                <svg class="w-8 h-8 text-warm-200" fill="currentColor" viewBox="0 0 20 20"><path d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" /></svg>
                            </div>
                            <div>
                                <h4 class="text-xl font-bold">"Estado de Solicitud"</h4>
                                <p class="text-sm text-warm-400 italic">"No hay solicitudes activas en este momento."</p>
                            </div>
                        </div>
                    </div>
                </div>

                <aside class="space-y-8">
                    <div class="bg-warm-900 rounded-[2.5rem] p-10 text-white space-y-6 shadow-2xl shadow-warm-900/40">
                        <h3 class="text-[10px] font-black uppercase tracking-widest text-warm-400">"Seguridad BFF"</h3>
                        <p class="text-sm font-light leading-relaxed opacity-80">
                            "Tus transacciones están protegidas por una arquitectura de servidor blindada. Tus datos nunca tocan el navegador."
                        </p>
                        <button class="w-full py-4 bg-white/10 border border-white/20 text-white text-[10px] font-black uppercase tracking-widest hover:bg-white hover:text-warm-900 transition-all rounded-2xl">
                            "Desconectar"
                        </button>
                    </div>
                </aside>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
