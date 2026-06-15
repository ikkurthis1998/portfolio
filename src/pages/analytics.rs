use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnalyticsStatsPublic {
    pub total_visitors: i64,
    pub unique_visitors: i64,
    pub active_now: i64,
    pub top_countries: Vec<(String, i64)>,
    pub top_domains: Vec<(String, i64)>,
    pub recent_visits: Vec<VisitRecordPublic>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VisitRecordPublic {
    pub time: String,
    pub ip: String,
    pub path: String,
    pub domain: String,
    pub country: String,
    pub device: String,
}

#[server(FetchAnalytics, "/api")]
pub async fn fetch_analytics() -> Result<AnalyticsStatsPublic, ServerFnError> {
    let db_client = db::get_db().await?;
    let stats = db::get_analytics(&db_client).await.map_err(|e| ServerFnError::new(e))?;

    // Convert to public struct
    let recent_visits = stats.recent_visits.into_iter().map(|v| VisitRecordPublic {
        time: v.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        ip: v.ip_address,
        path: v.path,
        domain: v.domain,
        country: v.country,
        device: if v.user_agent.contains("Mobile") { "Mobile".to_string() } else { "Desktop".to_string() },
    }).collect();

    Ok(AnalyticsStatsPublic {
        total_visitors: stats.total_visitors,
        unique_visitors: stats.unique_visitors,
        active_now: stats.active_now,
        top_countries: stats.top_countries,
        top_domains: stats.top_domains,
        recent_visits,
    })
}

#[component]
pub fn Analytics() -> impl IntoView {
    let analytics_resource = Resource::new(|| (), |_| fetch_analytics());

    view! {
        <div class="min-h-screen bg-slate-50 pt-28 pb-12 font-sans">
            <div class="max-w-7xl mx-auto px-6">
                
                // Header
                <div class="mb-10">
                    <p class="text-blue-600 font-medium mb-2 text-sm uppercase tracking-wide">"Dashboard"</p>
                    <h1 class="text-3xl md:text-4xl font-bold text-slate-900">"Visitor Analytics"</h1>
                </div>

                <Suspense fallback=move || view! { 
                    <div class="flex justify-center py-20">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                    </div> 
                }>
                    {move || {
                        analytics_resource.get().map(|result| match result {
                            Ok(stats) => view! {
                                // Top Stats Cards
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                                    <StatCard title="Total Visitors" value=stats.total_visitors.to_string() icon=view! {
                                        <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                                    }.into_any()/>
                                    <StatCard title="Unique Visitors" value=stats.unique_visitors.to_string() icon=view! {
                                        <svg class="w-6 h-6 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
                                    }.into_any()/>
                                    <StatCard title="Active Now" value=stats.active_now.to_string() icon=view! {
                                        <svg class="w-6 h-6 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                                    }.into_any()/>
                                </div>

                                // Middle Section: Domains & Countries
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                                    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
                                        <h3 class="text-lg font-bold text-slate-900 mb-4 border-b border-slate-100 pb-2">"Top Domains"</h3>
                                        <ul class="space-y-3">
                                            {stats.top_domains.into_iter().map(|(d, c)| view! {
                                                <li class="flex justify-between items-center group">
                                                    <span class="text-slate-700 font-medium group-hover:text-blue-600 transition-colors">{d}</span>
                                                    <span class="px-2.5 py-0.5 rounded-full bg-slate-100 text-slate-600 text-xs font-bold">{c}</span>
                                                </li>
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
                                        <h3 class="text-lg font-bold text-slate-900 mb-4 border-b border-slate-100 pb-2">"Top Countries"</h3>
                                        <ul class="space-y-3">
                                            {stats.top_countries.into_iter().map(|(c_code, c)| view! {
                                                <li class="flex justify-between items-center group">
                                                    <span class="text-slate-700 font-medium">{c_code}</span>
                                                    <span class="px-2.5 py-0.5 rounded-full bg-slate-100 text-slate-600 text-xs font-bold">{c}</span>
                                                </li>
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                </div>

                                // Recent Visitor Table
                                <div class="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
                                    <div class="px-6 py-5 border-b border-slate-200 bg-slate-50/50">
                                        <h3 class="text-lg font-bold text-slate-900">"Recent Visitors"</h3>
                                    </div>
                                    <div class="overflow-x-auto">
                                        <table class="w-full text-left text-sm">
                                            <thead class="bg-slate-50 text-slate-500 uppercase tracking-wider font-semibold text-xs border-b border-slate-200">
                                                <tr>
                                                    <th class="px-6 py-3">"Time"</th>
                                                    <th class="px-6 py-3">"Country"</th>
                                                    <th class="px-6 py-3">"Domain"</th>
                                                    <th class="px-6 py-3">"Path"</th>
                                                    <th class="px-6 py-3">"Device"</th>
                                                </tr>
                                            </thead>
                                            <tbody class="divide-y divide-slate-200 bg-white">
                                                {stats.recent_visits.into_iter().map(|v| view! {
                                                    <tr class="hover:bg-slate-50 transition-colors">
                                                        <td class="px-6 py-4 text-slate-600 whitespace-nowrap">{v.time}</td>
                                                        <td class="px-6 py-4">
                                                            <span class="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-slate-100 text-slate-700">
                                                                {v.country}
                                                            </span>
                                                        </td>
                                                        <td class="px-6 py-4 text-slate-900 font-medium">{v.domain}</td>
                                                        <td class="px-6 py-4 text-blue-600 font-mono text-xs">{v.path}</td>
                                                        <td class="px-6 py-4 text-slate-500">{v.device}</td>
                                                    </tr>
                                                }).collect::<Vec<_>>()}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            }.into_any(),
                            Err(e) => view! { 
                                <div class="p-6 bg-red-50 text-red-700 rounded-xl border border-red-200">
                                    "Error loading analytics: " {e.to_string()}
                                </div> 
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: String, icon: AnyView) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200 flex items-center justify-between hover:shadow-md transition-shadow">
            <div>
                <h3 class="text-slate-500 text-xs font-bold uppercase tracking-wider mb-1">{title}</h3>
                <p class="text-3xl font-bold text-slate-900">{value}</p>
            </div>
            <div class="p-3 bg-slate-50 rounded-lg">
                {icon}
            </div>
        </div>
    }
}
