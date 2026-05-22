import { Hero } from "@/components/sections/Hero";
import { Problem } from "@/components/sections/Problem";
import { HowItWorks } from "@/components/sections/HowItWorks";
import { Features } from "@/components/sections/Features";
import { ProductPreview } from "@/components/sections/ProductPreview";
import { UseCases } from "@/components/sections/UseCases";
import { ForDevelopers } from "@/components/sections/ForDevelopers";
import { CtaFooter } from "@/components/sections/CtaFooter";

export default function Home() {
  return (
    <main>
      <Hero />
      <Problem />
      <HowItWorks />
      <Features />
      <ProductPreview />
      <UseCases />
      <ForDevelopers />
      <CtaFooter />
    </main>
  );
}
