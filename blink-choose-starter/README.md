# Solana Blink Starter Pokémon

## Table of Contents
- [Project Overview](#project-overview)
- [Features](#features)
- [How It Works](#how-it-works)
- [Getting Started](#getting-started)
- [Technical Details](#technical-details)
- [Env Example](#env-example)


## Project Overview

This is a Solana-based project that allows users to choose starter Pokémon as NFTs. This project demonstrates how to use blinks and actions on solana.

## Features

- Choose from three starter Pokémon: Charmander, Squirtle, and Bulbasaur
- Each Pokémon has its own unique NFT associated with it
- Uses Solana's blockchain technology for minting and managing NFTs
- Runs on the devnet network
- Users can interact with the project by visiting dial.to and entering the site's URL + "/api/choose"
- Covers transaction fees for minting NFTs using a predefined wallet account

## How It Works

1. Users visit dial.to and navigate to the specified API endpoint
2. The project uses Solana's blockchain technology to mint NFTs for chosen Pokémon
3. Each NFT is associated with a unique wallet account
4. Users need sufficient funds in their wallets to cover transaction/gas fees.

## Getting Started

To run this project locally:

1. Clone the repository:

2. Install dependencies:

3. Start the development server:

4. Open your web browser and navigate to `http://localhost:3000`

## Technical Details

- Frontend: Built using Next.js
- Blockchain: Solana
- Network: Devnet
- Wallet Account: FyMeRo1DnkhSSQLEAmH8CyvMTXMFEvUddQ6aH5F4U5YM (covers minting costs)
- API Endpoint: dial.to + "/api/choose"

## Env example
Enter your wallet secret key in the .env file to cover the minting cost.
```
wallet=[..] 
```