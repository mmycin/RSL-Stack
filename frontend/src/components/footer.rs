use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-gray-300 p-8 mt-auto">
            <div class="container mx-auto">
                <div class="flex flex-wrap justify-between">
                    <div class="w-full md:w-1/3 mb-6 md:mb-0">
                        <h3 class="text-white text-lg font-bold mb-2">RSL-Stack</h3>
                        <p class="text-sm">A modern full-stack application built with Rust, SQLite, and Leptos.</p>
                    </div>
                    <div class="w-full md:w-1/3 mb-6 md:mb-0">
                        <h3 class="text-white text-lg font-bold mb-2">Quick Links</h3>
                        <ul class="space-y-2">
                            <li><a href="/" class="hover:text-white">Home</a></li>
                            <li><a href="/about" class="hover:text-white">About</a></li>
                            <li><a href="/contact" class="hover:text-white">Contact</a></li>
                        </ul>
                    </div>
                    <div class="w-full md:w-1/3">
                        <h3 class="text-white text-lg font-bold mb-2">Connect</h3>
                        <p class="text-sm">Follow us on social media for updates and news.</p>
                    </div>
                </div>
                <div class="border-t border-gray-700 mt-8 pt-6 text-sm text-center">
                    <p>&copy; {2024} RSL-Stack. All rights reserved.</p>
                </div>
            </div>
        </footer>
    }
}