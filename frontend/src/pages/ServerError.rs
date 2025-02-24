use leptos::*;
use crate::components::Navbar;
use crate::components::Footer;

#[component]
pub fn ServerError() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <Navbar />
            <div class="flex-grow container mx-auto px-4 py-8">
                <div class="text-center">
                    <h1 class="text-6xl font-bold text-gray-800 mb-4">500</h1>
                    <h2 class="text-2xl font-semibold text-gray-600 mb-4">Server Error</h2>
                    <p class="text-gray-500 mb-8">Sorry, something went wrong on our end. Please try again later.</p>
                    <a href="/" class="inline-block bg-blue-500 text-white px-6 py-3 rounded-lg hover:bg-blue-600 transition-colors">
                        Return to Home
                    </a>
                </div>
            </div>
            <Footer />
        </div>
    }
}