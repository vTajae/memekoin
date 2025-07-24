use leptos::prelude::*;

#[component]
pub fn MarketPage() -> impl IntoView {
    view! {
        <div class="container mx-auto py-8 px-4">
            <h1 class="text-4xl font-bold text-gray-900 mb-8">"Market Data"</h1>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">"BTC"</h3>
                    <div class="text-2xl font-bold text-gray-900 mb-1">"$45,000"</div>
                    <div class="text-sm font-medium text-green-600">"+2.5%"</div>
                </div>
                
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">"ETH"</h3>
                    <div class="text-2xl font-bold text-gray-900 mb-1">"$3,200"</div>
                    <div class="text-sm font-medium text-red-600">"-1.2%"</div>
                </div>
                
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">"DOGE"</h3>
                    <div class="text-2xl font-bold text-gray-900 mb-1">"$0.08"</div>
                    <div class="text-sm font-medium text-green-600">"+15.3%"</div>
                </div>
                
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">"SHIB"</h3>
                    <div class="text-2xl font-bold text-gray-900 mb-1">"$0.000025"</div>
                    <div class="text-sm font-medium text-red-600">"-5.7%"</div>
                </div>
            </div>
        </div>
    }
}
