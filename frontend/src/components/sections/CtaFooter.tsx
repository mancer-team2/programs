"use client";

import { motion } from "framer-motion";
import { ArrowUpRight, XLogo, DiscordLogo } from "@phosphor-icons/react";

export function CtaFooter() {
  return (
    <>
      {/* CTA Section */}
      <section className="relative min-h-[80dvh] bg-[#f8f8fa] px-6 md:px-10 py-24 md:py-32 overflow-hidden">
        {/* Background 3D ring image */}
        <div
          className="absolute top-0 right-0 w-[50%] h-full bg-contain bg-right bg-no-repeat opacity-60 hidden lg:block"
          style={{ backgroundImage: "url('/cta-bg.png')" }}
        />

        <div className="relative z-10 max-w-[1400px] mx-auto">
          {/* Headline */}
          <motion.h2
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5, delay: 0.1 }}
            className="text-5xl md:text-6xl lg:text-7xl font-bold text-zinc-900 leading-[1.08] tracking-tighter max-w-3xl"
          >
            Ready to distribute
            <br />
            tokens{" "}
            <span className="">
              the right way?
            </span>
          </motion.h2>

          {/* Subtitle */}
          <motion.p
            initial={{ opacity: 0, y: 16 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.4, delay: 0.2 }}
            className="mt-6 text-base text-zinc-500 max-w-[44ch] leading-relaxed"
          >
            Join leading teams building trust through transparency, security, and decentralization on{" "}
            <span className="">Solana</span>.
          </motion.p>

          {/* CTAs */}
          <motion.div
            initial={{ opacity: 0, y: 12 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.4, delay: 0.3 }}
            className="flex items-center gap-4 mt-10"
          >
            <a
              href="#"
              className="inline-flex items-center gap-2 px-7 py-3.5 bg-gradient-to-r from-violet-600 to-purple-600 hover:from-violet-500 hover:to-purple-500 text-white text-sm font-semibold rounded-lg active:scale-[0.97] active:-translate-y-[1px] transition-all"
            >
              Launch App <ArrowUpRight size={14} weight="bold" />
            </a>
            <a
              href="#"
              className="inline-flex items-center px-7 py-3.5 border border-zinc-300 hover:border-zinc-400 text-zinc-700 text-sm font-medium rounded-lg active:scale-[0.97] transition-all bg-white/60 backdrop-blur-sm"
            >
              Explore Docs
            </a>
          </motion.div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-zinc-200 bg-[#f8f8fa] px-6 md:px-10 py-6">
        <div className="max-w-[1400px] mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
          {/* Left — brand + copyright */}
          <div className="flex items-center gap-4">
            <span className="text-base font-bold text-zinc-900">Qior</span>
            <span className="text-xs text-zinc-400">© 2024 Qior Labs, Inc. All rights reserved.</span>
          </div>

          {/* Right — nav + socials */}
          <div className="flex items-center gap-8">
            <div className="flex gap-6 text-sm text-zinc-600">
              <a href="#" className="hover:text-zinc-900 transition-colors">Product</a>
              <a href="#" className="hover:text-zinc-900 transition-colors">Docs</a>
              <a href="#developers" className="hover:text-zinc-900 transition-colors">Developers</a>
              <a href="#" className="hover:text-zinc-900 transition-colors">About</a>
            </div>
            <div className="flex items-center gap-3 text-zinc-500">
              <a href="#" className="hover:text-zinc-900 transition-colors">
                <XLogo size={18} weight="bold" />
              </a>
              <a href="#" className="hover:text-zinc-900 transition-colors">
                <DiscordLogo size={18} weight="bold" />
              </a>
            </div>
          </div>
        </div>
      </footer>
    </>
  );
}
