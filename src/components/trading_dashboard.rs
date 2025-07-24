use leptos::prelude::*;
use super::*;

#[component]
pub fn TradingDashboard() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <TradingCard 
                title="📊 Market Overview"
                description="Real-time market data and analytics"
            >
                <MarketOverview />
            </TradingCard>
            
            <TradingCard 
                title="💹 Trading"
                description="Execute trades with advanced algorithms"
            >
                <TradingInterface />
            </TradingCard>
            
            <TradingCard 
                title="📈 Portfolio"
                description="Track your investments and performance"
            >
                <PortfolioDisplay />
            </TradingCard>
        </div>
    }
}
