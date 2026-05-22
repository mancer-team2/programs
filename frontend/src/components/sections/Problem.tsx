"use client";

import { motion } from "framer-motion";
import { TrendDown, EyeSlash, Warning } from "@phosphor-icons/react";

const problems = [
  {
    icon: TrendDown,
    title: "Dumping risk",
    desc: "Recipients can sell early, tanking token price and community trust.",
  },
  {
    icon: EyeSlash,
    title: "No transparency",
    desc: "Opaque allocations and vesting terms create distrust and FUD.",
  },
  {
    icon: Warning,
    title: "Operational risk",
    desc: "Manual processes lead to errors, delays, and compliance issues.",
  },
];

export function Problem() {
  return (
    <section className="relative min-h-[100dvh] bg-[#f8f8fa] overflow-hidden">
      {/* Background image — right side 3D crystal */}
      <div
        className="absolute top-0 right-0 w-[55%] h-full bg-cover bg-center bg-no-repeat hidden lg:block"
        style={{ backgroundImage: "url('/problem-bg.png')" }}
      />

      <div className="relative z-10 max-w-[1400px] mx-auto px-6 md:px-10 py-24 md:py-32">
        {/* Content — left column */}
        <div className="max-w-xl">
          <motion.h2
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5, delay: 0.1 }}
            className="text-5xl md:text-6xl lg:text-7xl font-bold text-zinc-900 leading-[1.08] tracking-tighter"
          >
            Manual token
            <br />
            distribution is{" "}
            <span className="">
              risky.
            </span>
          </motion.h2>

          {/* Pain points */}
          <div className="flex flex-col gap-8 mt-14">
            {problems.map((item, i) => (
              <motion.div
                key={item.title}
                initial={{ opacity: 0, y: 16 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.4, delay: 0.2 + i * 0.1 }}
                className="flex items-start gap-4"
              >
                <div className="shrink-0 w-14 h-14 rounded-2xl border border-zinc-200 bg-white flex items-center justify-center text-zinc-500">
                  <item.icon size={24} weight="light" />
                </div>
                <div className="pt-1">
                  <h3 className="text-base font-semibold text-zinc-900">{item.title}</h3>
                  <p className="text-sm text-zinc-500 mt-1 leading-relaxed max-w-[32ch]">
                    {item.desc}
                  </p>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
