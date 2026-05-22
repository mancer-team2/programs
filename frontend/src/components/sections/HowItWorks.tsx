"use client";

import { motion } from "framer-motion";
import { LockSimple, Clock, CheckSquare } from "@phosphor-icons/react";

const steps = [
  { icon: LockSimple, num: "1", title: "Lock", desc: "Creator locks tokens in escrow." },
  { icon: Clock, num: "2", title: "Vest", desc: "Smart contract releases over time." },
  { icon: CheckSquare, num: "3", title: "Claim", desc: "Recipients withdraw vested tokens." },
];

export function HowItWorks() {
  return (
    <section
      id="how-it-works"
      className="relative min-h-[100dvh] bg-gradient-to-b from-[#f8f8fa] to-[#f0eef5] px-6 md:px-10 py-24 md:py-32 overflow-hidden"
    >
      <div className="max-w-[1400px] mx-auto">
        {/* Headline */}
        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.1 }}
          className="text-5xl md:text-6xl lg:text-7xl font-bold text-zinc-900 tracking-tighter leading-[1.08]"
        >
          Lock.{" "}
          <span className="">
            Vest. Claim.
          </span>
        </motion.h2>

        {/* Steps — horizontal flow */}
        <div className="relative mt-20 md:mt-28">
          {/* Connector line */}
          <div className="hidden md:block absolute top-[72px] left-[16%] right-[16%] h-px bg-gradient-to-r from-violet-300 via-cyan-300 to-cyan-300 z-0" />
          {/* Connector dots */}
          <div className="hidden md:block absolute top-[68px] left-[calc(33%-1px)] w-2.5 h-2.5 rounded-full border-2 border-violet-300 bg-white z-10" />
          <div className="hidden md:block absolute top-[68px] left-[calc(66%-1px)] w-2.5 h-2.5 rounded-full border-2 border-cyan-300 bg-white z-10" />

          <div className="grid grid-cols-1 md:grid-cols-3 gap-16 md:gap-8">
            {steps.map((step, i) => (
              <motion.div
                key={step.num}
                initial={{ opacity: 0, y: 24 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: 0.2 + i * 0.12 }}
                className="flex flex-col items-center text-center relative z-10"
              >
                {/* Circular icon with rings */}
                <div className="relative w-36 h-36 flex items-center justify-center">
                  {/* Outer ring */}
                  <div className="absolute inset-0 rounded-full border border-zinc-200" />
                  {/* Middle ring */}
                  <div className="absolute inset-3 rounded-full border border-zinc-200/80 bg-white/40" />
                  {/* Inner circle */}
                  <div className="w-16 h-16 rounded-2xl bg-white border border-zinc-200 shadow-[0_4px_20px_-4px_rgba(124,58,237,0.12)] flex items-center justify-center text-violet-500">
                    <step.icon size={28} weight="duotone" />
                  </div>
                </div>

                {/* Label */}
                <div className="mt-6 flex items-center gap-2">
                  <span className="text-sm font-mono text-violet-500">{step.num}</span>
                  <h3 className="text-lg font-semibold text-zinc-900">{step.title}</h3>
                </div>
                <p className="text-sm text-zinc-500 mt-2 leading-relaxed max-w-[20ch]">
                  {step.desc}
                </p>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
