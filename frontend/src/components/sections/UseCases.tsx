"use client";

import { motion } from "framer-motion";
import { UsersThree, TrendUp, Gift, GlobeSimple } from "@phosphor-icons/react";

const cases = [
  { icon: UsersThree, title: "Teams &\nFounders" },
  { icon: TrendUp, title: "Investors &\nAdvisors" },
  { icon: Gift, title: "Grants &\nRewards" },
  { icon: GlobeSimple, title: "DAOs &\nCommunities" },
];

export function UseCases() {
  return (
    <section
      id="use-cases"
      className="relative min-h-[100dvh] bg-[#f8f8fa] px-6 md:px-10 py-24 md:py-32 overflow-hidden"
    >
      {/* Right-side 3D portal image */}
      <div
        className="absolute top-0 right-0 w-[50%] h-full bg-contain bg-right bg-no-repeat opacity-70 hidden lg:block"
        style={{ backgroundImage: "url('/usecases-bg.png')" }}
      />

      <div className="relative z-10 max-w-[1400px] mx-auto">
        {/* Headline */}
        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.1 }}
          className="text-5xl md:text-6xl lg:text-7xl font-bold text-zinc-900 leading-[1.08] tracking-tighter max-w-2xl"
        >
          Built for teams
          <br />
          and{" "}
          <span className="">
            communities.
          </span>
        </motion.h2>

        {/* Subtitle */}
        <motion.p
          initial={{ opacity: 0, y: 16 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.4, delay: 0.2 }}
          className="mt-6 text-base text-zinc-500 max-w-[40ch] leading-relaxed"
        >
          Qior adapts to the way you build, fund, and grow together onchain.
        </motion.p>

        {/* Use case icons row */}
        <div className="flex flex-wrap gap-6 mt-14 max-w-xl">
          {cases.map((c, i) => (
            <motion.div
              key={c.title}
              initial={{ opacity: 0, y: 16 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: 0.25 + i * 0.08 }}
              className="flex flex-col items-center gap-3 w-[calc(25%-18px)] min-w-[100px]"
            >
              <div className="w-16 h-16 rounded-2xl bg-white border border-zinc-200/80 shadow-[0_2px_8px_-2px_rgba(124,58,237,0.06)] flex items-center justify-center text-violet-500">
                <c.icon size={28} weight="light" />
              </div>
              <p className="text-sm font-medium text-zinc-700 text-center whitespace-pre-line leading-tight">
                {c.title}
              </p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
