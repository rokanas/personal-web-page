// components/project_item.rs
use yew::prelude::*;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct ProjectItemProps {
    pub image_src: String,
    pub title: String,
    pub description: String,
    pub github_url: String,
    #[prop_or_default]
    pub image_alt: Option<String>,
}

#[function_component(ProjectItem)]
pub fn project_item(props: &ProjectItemProps) -> Html {
    let github_click = {
        let github_url = props.github_url.clone();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let _ = window.open_with_url_and_target(&github_url, "_blank");
            }
        })
    };

    let alt_text = props.image_alt.as_ref()
        .unwrap_or(&props.title)
        .clone();

    html! {
        <div class="bg-gray-900 border-2 border-gray-700 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-105 max-w-sm">
            // Project Image
            <div class="aspect-video bg-gray-800 overflow-hidden">
                <img 
                    src={props.image_src.clone()}
                    alt={alt_text}
                    class="w-full h-full object-cover hover:scale-110 transition-transform duration-500 image-rendering-pixelated"
                    style="image-rendering: pixelated;"
                />
            </div>
            
            // Project Content
            <div class="p-6">
                // Title
                <h3 class="text-xl font-bold text-red-400 mb-3 font-mono">
                    {&props.title}
                </h3>
                
                // Description
                <p class="text-gray-300 mb-4 text-sm leading-relaxed">
                    {&props.description}
                </p>
                
                // GitHub Button
                <button 
                    onclick={github_click}
                    class="group w-full bg-gray-800 hover:bg-red-600 border-2 border-red-400 hover:border-red-300 text-red-400 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm"
                >
                    <div class="flex items-center justify-center gap-2">
                        <span>{"VIEW ON GITHUB"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </button>
            </div>
        </div>
    }
}