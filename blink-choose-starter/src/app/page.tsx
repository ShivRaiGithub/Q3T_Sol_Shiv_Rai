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
      <h1 className="text-1xl text-center z-10 hover:underline"><a href="https://dial.to/?action=solana-action:https://choose-starter-lac.vercel.app/api/choose">https://dial.to/?action=solana-action:https://choose-starter-lac.vercel.app/api/choose</a></h1>

      <div className="flex flex-col items-center gap-6 w-full max-w-7xl px-4 py-5 mx-auto z-10">
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

      <section className="mt-12 z-10 px-4">
        <h2 className="text-3xl font-bold m-4">About</h2>
        <p className="mb-4">
          This is a Solana-based project that allows users to choose starter Pokémon as NFTs.
        </p>
        <ul className="list-disc list-inside mb-4">
          <li>Choose from three starter Pokémon: Charmander, Squirtle, and Bulbasaur.</li>
          <li>Each Pokémon has its own unique NFT associated with it.</li>
          <li>The project uses Solana&apos;s blockchain technology to mint and manage these NFTs.</li>
          <li>
            Users can interact with the project by visiting:{" "}
            <a href="https://dial.to/?action=solana-action:https://choose-starter-lac.vercel.app/api/choose" className="text-blue-500 hover:underline">dial.to</a>
          </li>
          <li>The project runs on the devnet network.</li>
          <li>Ensure you have sufficient funds to cover transaction fees for minting NFTs.</li>
          <li>The project is using wallet account FyMeRo1DnkhSSQLEAmH8CyvMTXMFEvUddQ6aH5F4U5YM to cover minting costs.</li>
          <li>If you are unable to mint despite enough funds, try sending some devnet SOL to the wallet account above.</li>
        </ul>
      </section>
    </main>
  );
}
