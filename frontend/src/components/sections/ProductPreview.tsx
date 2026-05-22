"use client";

import { motion } from "framer-motion";
import { ArrowUpRight, House, Pulse, Coins, ChartLine, Gear, Plus } from "@phosphor-icons/react";

const navItems = [
  { icon: House, label: "Overview", active: false },
  { icon: Pulse, label: "Streams", active: true },
  { icon: Coins, label: "Distributions", active: false },
  { icon: ChartLine, label: "Vesting", active: false },
  { icon: ChartLine, label: "Analytics", active: false },
  { icon: Gear, label: "Settings", active: false },
];

const stats = [
  { label: "Total Streams", value: "24", sub: "Active streams" },
  { label: "Total Distributed", value: "1.42M", sub: "QIOR" },
  { label: "Total Recipients", value: "1,248", sub: "Unique wallets" },
  { label: "Avg. Completion", value: "68%", sub: "Across all streams" },
];

const streams = [
  { name: "Ecosystem Rewards", amount: "500,000 QIOR", distributed: "320,000 QIOR", pct: 64, status: "Active" },
  { name: "Community Incentives", amount: "250,000 QIOR", distributed: "125,000 QIOR", pct: 50, status: "Active" },
  { name: "Partner Program", amount: "200,000 QIOR", distributed: "80,000 QIOR", pct: 40, status: "Paused" },
  { name: "Liquidity Mining", amount: "300,000 QIOR", distributed: "210,000 QIOR", pct: 70, status: "Active" },
  { name: "Developer Grants", amount: "170,000 QIOR", distributed: "102,000 QIOR", pct: 60, status: "Active" },
];

export function ProductPreview() {
  return (
    <section className="relative min-h-[100dvh] bg-[#f8f8fa] px-6 md:px-10 py-24 md:py-32 overflow-hidden">
      {/* Bottom decorative image */}
      <div
        className="absolute bottom-0 left-0 w-[40%] h-[30%] bg-contain bg-left-bottom bg-no-repeat opacity-60 hidden lg:block"
        style={{ backgroundImage: "url('/product-bg.png')" }}
      />

      <div className="relative z-10 max-w-[1400px] mx-auto">
        <div className="grid grid-cols-1 lg:grid-cols-[1fr_1.6fr] gap-12 lg:gap-16 items-start">
          {/* Left — text */}
          <div className="pt-4">
            <motion.h2
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 0.1 }}
              className="text-5xl md:text-6xl font-bold text-zinc-900 leading-[1.08] tracking-tighter"
            >
              Simple interface.
              <br />
              Powerful{" "}
              <span className="text">
                control.
              </span>
            </motion.h2>

            <motion.p
              initial={{ opacity: 0, y: 16 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: 0.2 }}
              className="mt-6 text-base text-zinc-500 max-w-[36ch] leading-relaxed"
            >
              Manage streams, distribute tokens, and monitor performance — all from one intuitive dashboard.
            </motion.p>

          
          </div>

          {/* Right — dashboard mockup */}
          <motion.div
            initial={{ opacity: 0, y: 24 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6, delay: 0.15 }}
            className="rounded-2xl border border-zinc-200 bg-white shadow-[0_20px_60px_-15px_rgba(0,0,0,0.08)] overflow-hidden"
          >
            <div className="flex">
              {/* Sidebar */}
              <div className="hidden md:flex flex-col w-48 border-r border-zinc-100 py-5 px-4 gap-1">
                <div className="text-base font-bold text-zinc-900 px-3 mb-4">Qior</div>
                {navItems.map((item) => (
                  <div
                    key={item.label}
                    className={`flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm ${
                      item.active
                        ? "bg-violet-50 text-violet-600 font-medium"
                        : "text-zinc-500"
                    }`}
                  >
                    <item.icon size={16} weight={item.active ? "fill" : "regular"} />
                    {item.label}
                  </div>
                ))}
                <div className="mt-auto pt-6 px-3">
                  <div className="flex items-center gap-2 text-xs text-zinc-400">
                    <div className="w-2 h-2 rounded-full bg-emerald-400" />
                    Solana
                  </div>
                </div>
              </div>

              {/* Main content */}
              <div className="flex-1 p-5">
                {/* Header */}
                <div className="flex items-center justify-between mb-5">
                  <div>
                    <h3 className="text-lg font-semibold text-zinc-900">Streams</h3>
                    <p className="text-xs text-zinc-400 mt-0.5">Create and manage token streams across your ecosystem.</p>
                  </div>
                  <button className="flex items-center gap-1.5 px-3.5 py-2 bg-violet-600 text-white text-xs font-medium rounded-lg">
                    <Plus size={12} weight="bold" /> New Stream
                  </button>
                </div>

                {/* Stats row */}
                <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-6">
                  {stats.map((s) => (
                    <div key={s.label} className="border border-zinc-100 rounded-lg p-3">
                      <p className="text-[10px] text-zinc-400 uppercase tracking-wide">{s.label}</p>
                      <p className="text-lg font-bold text-zinc-900 font-mono mt-1">{s.value}</p>
                      <p className="text-[10px] text-zinc-400 mt-0.5">{s.sub}</p>
                    </div>
                  ))}
                </div>

                {/* Table */}
                <div>
                  <p className="text-sm font-semibold text-zinc-900 mb-3">All Streams</p>
                  <div className="text-[10px] text-zinc-400 uppercase tracking-wide grid grid-cols-[1.5fr_1fr_1fr_0.8fr_0.6fr] gap-2 px-2 pb-2 border-b border-zinc-100">
                    <span>Stream</span>
                    <span>Total Amount</span>
                    <span>Distributed</span>
                    <span>Progress</span>
                    <span>Status</span>
                  </div>
                  {streams.map((s) => (
                    <div
                      key={s.name}
                      className="grid grid-cols-[1.5fr_1fr_1fr_0.8fr_0.6fr] gap-2 items-center px-2 py-2.5 border-b border-zinc-50 text-xs"
                    >
                      <div className="flex items-center gap-2">
                        <div className="w-6 h-6 rounded-full bg-violet-100 flex items-center justify-center text-[9px] font-bold text-violet-600">
                          {s.name[0]}
                        </div>
                        <div>
                          <p className="text-zinc-900 font-medium text-[11px]">{s.name}</p>
                          <p className="text-[9px] text-zinc-400">QIOR</p>
                        </div>
                      </div>
                      <span className="text-zinc-600 font-mono text-[11px]">{s.amount}</span>
                      <span className="text-zinc-600 font-mono text-[11px]">{s.distributed}</span>
                      <div className="flex items-center gap-2">
                        <div className="flex-1 h-1 bg-zinc-100 rounded-full overflow-hidden">
                          <div className="h-full bg-violet-500 rounded-full" style={{ width: `${s.pct}%` }} />
                        </div>
                        <span className="text-zinc-500 font-mono text-[10px]">{s.pct}%</span>
                      </div>
                      <span
                        className={`text-[10px] font-medium ${
                          s.status === "Active" ? "text-emerald-500" : "text-amber-500"
                        }`}
                      >
                        {s.status}
                      </span>
                    </div>
                  ))}
                  <div className="flex items-center justify-between pt-3 text-[10px] text-zinc-400">
                    <span>Showing 1 to 5 of 24 streams</span>
                    <div className="flex items-center gap-1">
                      <span className="w-5 h-5 rounded bg-violet-600 text-white flex items-center justify-center text-[9px] font-bold">1</span>
                      <span className="w-5 h-5 rounded text-zinc-500 flex items-center justify-center">2</span>
                      <span className="w-5 h-5 rounded text-zinc-500 flex items-center justify-center">3</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
