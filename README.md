# Block-Exchange

## Vision
Block Exchange is a DeFi exchange for bonds and credit derivatives

## Description
The majority of fixed income market making, particularly for off-the-run bonds, happens through inter-dealer broker platforms that rely on voice and manual assistance. RFQs are received through phone calls and and confirmed electronically on pop-up windows that have an expiration timer of 5 minutes. Dealers are faced with 300-700 RFQ’s per day, resulting in a response rate of 30-40% and a hit rate of 10-12%. Desks must trade-off between accuracy and time, missing deals and facing losses on overall operational revenue. At Block Exchange, we recognize that automated market-making infrastructure can serve as an improved alternative to manual market making in fixed income markets.

Block Exchange's fixed income trading platform allows dealers to: leverage advanced models to monitor market conditions constantly; provide continuous liquidity by offering prices in real-time for both buying and selling of fixed income products; and reduce operational risk by eliminating the need for manual market making. The result is a fixed income exchange with the highest execution efficiency and lowest transaction costs for trading of bonds and credit derivatives.

What it does Block Exchange’s fixed-income trading platform is a hybrid decentralized exchange for bonds and credit derivatives. The exchange can receive RFQ’s, either directly or from electronic inter-dealer platforms, convert them into limit orders, and match them against anonymous all-to-all liquidity provided either directly to the order-book or on an automated market maker. Trades are executed with instant settlement and reported to regulatory reporting agencies in a compliant manner. Thus, it provides fixed-income traders with an exchange that offers several benefits including: faster execution, improved liquidity, increased price transparency, and reduced transaction costs.

How we built it We built a hybrid decentralized exchange on Injective for the trading of tokenized bonds and credit derivatives. Traders place market and limit orders that are processed on an order book and matched against liquidity on an automated market maker by a constant product model. Market makers can provide concentrated liquidity on the automated market maker or quote on the central limit order-book. The hybrid DEX's sequencer calculates a best execution range based on the liquidity present on the automated market maker and executes all orders that fall within that range in order of best price. This allows both traditional market makers and alternative liquidity providers to provide orders on Block Exchange systems, while fixed income traders can access this liquidity underneath a traditional exchange interface.

Challenges we ran into We ran into challenges with rebuilding our hybrid dex with concentrated liquidity provisioning capabilities as we had to rewrite our codebase from scratch using the Cosmos SDK. We think this was the best choice for the trading platform.

Accomplishments that we're proud of We are proud of completing our front-end interface as well as writing contracts for tokenized bond contracts, liquidity pool creation, and order placement mechanisms.

What we learned We learned that automated market making infrastructure can help fixed income dealers 4x their operational revenue and can improve execution efficiency / reduce transaction costs in fixed income trading.

What's next We are going to deploy our bond pairs onto a testnet where they can be traded and LP'd on - through Block Exchange's hybrid decentralized exchange platform. We will then tokenize treasury bills from fixed income dealers and offer bond yields to DeFi yield farmers on Cosmos. We will use this to create transaction cost and execution efficiency analyses for bonds traded on the Block Exchange hybrid DEX and use that to sell our trading platform to banks and fixed income dealers in traditional finance.