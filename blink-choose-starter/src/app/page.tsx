import Image from "next/image";

export default function Home() {
  return (
    <main className="relative flex flex-col items-center justify-center min-h-screen text-white bg-gradient-to-b from-gray-900 via-gray-800 to-gray-900">
      {/* Background columns */}
      <div className="absolute inset-0 flex">
        <div className="w-1/3 h-full bg-gradient-to-b from-red-500 via-red-600 to-black border-r border-black"></div>
        <div className="w-1/3 h-full bg-gradient-to-b from-green-500 via-green-600 to-black border-r border-black"></div>
        <div className="w-1/3 h-full bg-gradient-to-b from-blue-500 via-blue-600 to-black"></div>
      </div>

      {/* Content */}
      <h1 className="text-5xl font-bold mb-8 text-center z-10">Solana Blink Starter Pokémon</h1>

      <div className="flex flex-col items-center gap-6 w-full max-w-7xl px-4 py-12 mx-auto z-10">
        <div className="flex flex-col items-center justify-center bg-white rounded-lg p-8 shadow-md text-black">
          <div className="relative w-[33.33vw] h-[25vh]">
            <Image
              src="/pokeballs.jpg"
              alt="Pokeballs"
              layout="fill"
              objectFit="contain"
              className="rounded-lg"
            />
          </div>
        </div>
      </div>

      <section className="mt-12 z-10 text-center px-4">
        <h2 className="text-3xl font-bold mb-4">About</h2>
        <p className="mb-4">
          This is a Solana-based project that allows users to choose starter Pokémon as NFTs.
        </p>
        <ul className="list-disc list-inside mb-4">
          <li>Choose from three starter Pokémon: Charmander, Squirtle, and Bulbasaur.</li>
          <li>Each Pokémon has its own unique NFT associated with it.</li>
          <li>The project uses Solana&apos;s blockchain technology to mint and manage these NFTs.</li>
          <li>
            Users can interact with the project by visiting:
            <a href="https://dial.to" className="text-blue-500 hover:underline">dial.to</a> and entering the site&apos;s URL + &quot;/api/choose&quot;.
          </li>
          <li>The project runs on the devnet network.</li>
          <li>Ensure you have sufficient funds to cover transaction fees for minting NFTs.</li>
          <li>Use wallet account FyMeRo1DnkhSSQLEAmH8CyvMTXMFEvUddQ6aH5F4U5YM to cover minting costs.</li>
          <li>If you encounter issues, try sending some devnet SOL to the wallet account.</li>
        </ul>
      </section>
    </main>
  );
}
