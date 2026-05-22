"use client";

import { motion } from "framer-motion";
import { ShieldCheck, Cube, FileTs, BookOpen } from "@phosphor-icons/react";

const code = `import { Qior } from '@qior/sdk';

const qior = new Qior({
  apiKey: process.env.QIOR_API_KEY,
  network: 'mainnet',
});

// Create escrow
const escrow = await qior.escrow.create({
  maker: '0x8F...A3c1',
  amount: '100000000', // 1.0 USDC
  token: 'USDC',
  resolver: '0x3B...7F9e',
  metadata: {
    orderId: '#1024',
    description: 'Digital Asset Escrow',
  }
});

console.log('Escrow created:', escrow.id);`;

const devFeatures = [
  { icon: ShieldCheck, title: "Audited Escrow", desc: "Smart contracts audited by leading security firms." },
  { icon: FileTs, title: "TypeScript SDK", desc: "Build faster with a modern, type-safe developer experience." },
  { icon: Cube, title: "On-Chain Transparency", desc: "Every transaction is verifiable and publicly auditable." },
  { icon: BookOpen, title: "Read the Docs", desc: "Comprehensive guides and API references." },
];

export function ForDevelopers() {
  return (
    <section id="developers" className="relative bg-[#07060b] px-6 md:px-10 py-24 md:py-32 overflow-hidden">
      {/* Bottom decorative bg */}
      <div
        className="absolute bottom-0 right-0 w-[40%] h-[50%] bg-contain bg-right-bottom bg-no-repeat opacity-50 hidden lg:block"
        style={{ backgroundImage: "url('/dev-bg.png')" }}
      />

      <div className="relative z-10 max-w-[1400px] mx-auto">
        <div className="grid grid-cols-1 lg:grid-cols-[1.1fr_1fr] gap-12 lg:gap-16 items-start">
          {/* Left */}
          <div>
            {/* Headline */}
            <motion.h2
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 0.1 }}
              className="text-5xl md:text-6xl lg:text-7xl font-bold text-white leading-[1.08] tracking-tighter"
            >
              Built for{" "}
              <span className="">
                trust.
              </span>
              <br />
              Made for{" "}
              <span className="">
                builders.
              </span>
            </motion.h2>

            {/* Subtitle */}
            <motion.p
              initial={{ opacity: 0, y: 16 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4, delay: 0.2 }}
              className="mt-6 text-base text-zinc-400 max-w-[42ch] leading-relaxed"
            >
              Qior secures value with audited protocols and gives developers everything they need to build with confidence.
            </motion.p>

            {/* 2x2 feature grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mt-12">
              {devFeatures.map((f, i) => (
                <motion.div
                  key={f.title}
                  initial={{ opacity: 0, y: 16 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.4, delay: 0.25 + i * 0.08 }}
                  className="flex items-start gap-3"
                >
                  <div className="shrink-0 w-11 h-11 rounded-xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-violet-400">
                    <f.icon size={20} weight="duotone" />
                  </div>
                  <div>
                    <h3 className="text-sm font-semibold text-white">{f.title}</h3>
                    <p className="text-xs text-zinc-500 mt-0.5 leading-relaxed">{f.desc}</p>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>

          {/* Right — code editor */}
          <motion.div
            initial={{ opacity: 0, y: 24 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6, delay: 0.15 }}
            className="w-full rounded-xl border border-zinc-800 bg-[#0d0c14] overflow-hidden"
          >
            {/* Tab bar */}
            <div className="flex items-center justify-between px-5 py-3 border-b border-zinc-800">
              <div className="flex gap-5 text-sm">
                <span className="text-white font-medium border-b border-violet-500 pb-2">Node.js</span>
                <span className="text-zinc-500">TypeScript</span>
                <span className="text-zinc-500">cURL</span>
              </div>
              <span className="text-xs text-zinc-600">Copy</span>
            </div>

            {/* Code with line numbers */}
            <div className="flex overflow-x-auto">
              <div className="py-5 px-3 text-right select-none">
                {code.split("\n").map((_, i) => (
                  <div key={i} className="text-[11px] text-zinc-700 font-mono leading-[1.7]">
                    {i + 1}
                  </div>
                ))}
              </div>
              <pre className="py-5 pr-5 text-[12px] text-zinc-300 font-mono leading-[1.7] overflow-x-auto">
                <code>{code}</code>
              </pre>
            </div>

            {/* Status bar */}
            <div className="flex items-center justify-between px-5 py-2.5 border-t border-zinc-800 text-[10px] text-zinc-600">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-emerald-400" />
                All systems secure
              </div>
              <span>v1.2.0</span>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
