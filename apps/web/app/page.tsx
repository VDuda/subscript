import Image from "next/image";
import { Button } from "@repo/ui/button";
import Link from "next/link";
import { SubscribeForm } from "../components/SubscribeForm";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="flex flex-col items-center justify-center space-y-4">
        <h1 className="text-4xl font-bold">Welcome to Subscript</h1>
        <p className="text-xl">Choose your path:</p>
        <div className="flex space-x-4">
          <Link href="/merchant" passHref>
            <Button appName="web">Go to Merchant Page</Button>
          </Link>
          <Link href="/subscriber" passHref>
            <Button appName="web">Go to Subscriber Page</Button>
          </Link>
        </div>
        <div className="mt-8">
          <SubscribeForm />
        </div>
      </div>
    </main>
  );
}
