"use client";

import { motion } from "framer-motion";
import { ChartLineUp, ShieldCheck, Eye, ArrowCounterClockwise } from "@phosphor-icons/react";

const features = [
  {
    icon: ChartLineUp,
    title: "Cliff & Linear Vesting",
    desc: "Create cliff and linear vesting schedules with flexible parameters.",
  },
  {
    icon: ShieldCheck,
    title: "Secure Escrow",
    desc: "Tokens are held in audited escrow contracts until conditions are met.",
  },
  {
    icon: Eye,
    title: "Transparent On-Chain",
    desc: "All vesting events and releases are verifiable on-chain.",
  },
  {
    icon: ArrowCounterClockwise,
    title: "Revocable Streams",
    desc: "Pause or revoke streams securely with multi-sig controls.",
  },
];

export function Features() {
  return (
    <section id="features" className="relative bg-[#0a0a12] px-6 md:px-10 py-24 md:py-32 overflow-hidden">
      {/* Background 3D image — top right */}
      <div
        className="absolute top-0 right-0 w-[45%] h-[60%] bg-contain bg-right-top bg-no-repeat opacity-80 hidden lg:block"
        style={{ backgroundImage: "url('/features-bg.png')" }}
      />

      <div className="relative z-10 max-w-[1400px] mx-auto">
        {/* Headline */}
        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.1 }}
          className="text-5xl md:text-6xl lg:text-7xl font-bold text-white leading-[1.08] tracking-tighter max-w-3xl"
        >
          Everything you need
          <br />
          for{" "}
          <span className="">
            token distribution.
          </span>
        </motion.h2>

        {/* Feature cards — 4 columns */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 mt-20">
          {features.map((f, i) => (
            <motion.div
              key={f.title}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: 0.2 + i * 0.08 }}
              className="flex flex-col gap-5"
            >
              {/* Icon box */}
              <div className="w-14 h-14 rounded-xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-violet-400">
                <f.icon size={26} weight="duotone" />
              </div>

              {/* Text */}
              <h3 className="text-lg font-semibold text-white">{f.title}</h3>
              <p className="text-sm text-zinc-500 leading-relaxed">{f.desc}</p>

              {/* Accent underline */}
              <div className="w-8 h-0.5 rounded-full bg-violet-500 mt-auto" />
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
