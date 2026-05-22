"use client";

import { motion } from "framer-motion";
import { ArrowUpRight } from "@phosphor-icons/react";

export function Hero() {
  return (
    <section className="relative min-h-[100dvh] overflow-hidden bg-[#07060b]">
      {/* Background image layer */}
      <div
        className="absolute inset-0 bg-cover bg-center bg-no-repeat opacity-70"
        style={{ backgroundImage: "url('/hero-bg.png')" }}
      />
      {/* Gradient overlays */}
      <div className="absolute inset-0 bg-gradient-to-br from-[#0d0b14]/90 via-[#0a0a0f]/60 to-[#120b1e]/80" />
      <div className="absolute inset-0 bg-gradient-to-t from-[#07060b] via-transparent to-transparent" />

      {/* Nav */}
      <nav className="relative z-10 flex items-center justify-between px-6 md:px-10 py-6 max-w-[1400px] mx-auto">
        <span className="text-xl font-bold tracking-tight text-white">◆ Qior</span>
        <div className="hidden md:flex items-center gap-10 text-sm text-zinc-300">
          <a href="#features" className="hover:text-white transition-colors">Product</a>
          <a href="#developers" className="hover:text-white transition-colors">Docs</a>
          <a href="#developers" className="hover:text-white transition-colors">Developers</a>
          <a href="#use-cases" className="hover:text-white transition-colors">About</a>
        </div>
        <a
          href="#"
          className="hidden md:inline-flex items-center gap-1.5 px-5 py-2.5 text-sm font-medium text-white border border-zinc-700 rounded-lg hover:bg-white/5 active:scale-[0.97] transition-all"
        >
          Launch App <ArrowUpRight size={14} weight="bold" />
        </a>
      </nav>

      {/* Content */}
      <div className="relative z-10 max-w-[1400px] mx-auto px-6 md:px-10 pt-24 md:pt-32 pb-32">
        {/* Headline */}
        <motion.h1
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="text-5xl md:text-7xl lg:text-8xl font-bold text-white leading-[1.05] tracking-tighter max-w-3xl"
        >
          Distribute tokens
          <br />
          with{" "}
          <span className="">
            trust.
          </span>
        </motion.h1>

        {/* Subtitle */}
        <motion.p
          initial={{ opacity: 0, y: 16 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.35 }}
          className="mt-8 text-lg text-zinc-400 max-w-[50ch] leading-relaxed"
        >
          Lock tokens in escrow and release them through transparent vesting on{" "}
          <span className="">Solana</span>.
        </motion.p>

        {/* CTAs */}
        <motion.div
          initial={{ opacity: 0, y: 16 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.5 }}
          className="flex items-center gap-4 mt-10"
        >
          <a
            href="#"
            className="inline-flex items-center gap-2 px-7 py-3.5 bg-gradient-to-r from-violet-600 to-purple-600 hover:from-violet-500 hover:to-purple-500 text-white text-sm font-semibold rounded-lg active:scale-[0.97] active:-translate-y-[1px] transition-all"
          >
            Launch App <ArrowUpRight size={14} weight="bold" />
          </a>
          <a
            href="#features"
            className="inline-flex items-center px-7 py-3.5 border border-zinc-700 hover:border-zinc-500 text-white text-sm font-medium rounded-lg active:scale-[0.97] transition-all backdrop-blur-sm"
          >
            Explore Docs
          </a>
        </motion.div>
      </div>
    </section>
  );
}
