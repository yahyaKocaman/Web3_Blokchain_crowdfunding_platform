![Team_Logo](https://github.com/yahyaKocaman/Web3_Blokchain_crowdfunding_platform/blob/main/_dbb797c2-5e66-4b16-b142-04242e8a3450.jpeg)

# Crowdfunding Platform

## About Me
Yahya Kocaman is a software development student at Istanbul Aydın University. 
His interest in cryptocurrency began in 2022, leading him to explore the world of Web3 during the summer of 2024. 
Eager to deepen his knowledge, he participated in the Rise in Stellar Fullstack bootcamp, where he honed his skills in blockchain technology. 
Outside of his studies and projects, Yahya enjoys gaming, which fuels his passion for technology and innovation.
His journey reflects a commitment to leveraging his skills in the evolving digital landscape.

## Description

The Crowdfunding Platform is a decentralized application built on the Stellar Blockchain, enabling users to create and fund various projects. 
Users can easily launch their projects by providing details and funding goals. 
Supporters can browse projects and contribute funds, helping bring innovative ideas to life.

Key features include:

- **Project Creation:** Users can create projects with clear descriptions, funding goals, and timelines.
- **Project Funding:** Supporters can directly fund projects using Stellar's fast and secure transaction capabilities.
- **Automatic Fund Transfer:** Once a project's funding target is reached, funds are automatically transferred to the project creator, ensuring a smooth process.

This platform aims to empower entrepreneurs and creators by connecting them with potential supporters, making it easier to turn dreams into reality.

## Vision

The Crowdfunding Platform envisions a vibrant ecosystem where entrepreneurs and creators can transform their ideas into reality. 
By providing a space for users to create and support projects, we empower individuals to pursue their passions and make a positive impact on their communities. 
Our platform fosters collaboration and connection, allowing users to contribute to meaningful goals set by fellow community members. 
We believe that through collective effort and support, we can unlock the potential of innovative ideas, driving social and economic growth while inspiring others to dream big and take action.

## Project RoadMap / Future Plans

### Software Development Plan for Crowdfunding Platform

**1. Define Smart Contract Specifications**  
   - Identify key features and functionalities for the smart contract, including:
     - **Variables:**
       - Project ID
       - Project creator address
       - Funding goal
       - Current funds raised
       - Project deadline
       - Status (active, completed, or failed)
     - **Functions:**
       - `createProject()`: Allows users to create a new project.
       - `fundProject()`: Enables users to contribute funds to a project.
       - `checkFundingStatus()`: Returns the current funding amount and status.
       - `completeProject()`: Automatically transfers funds to the creator if the goal is met.
       - `refund()`: Allows refunds if the funding goal is not reached by the deadline.

**2. Develop Smart Contracts**  
   - Implement the smart contract using Rust for the Stellar blockchain, incorporating the defined functions and variables.  
   - Write unit tests to ensure the contract works as expected.

**3. Build the Front-End Interface**  
   - Create a user-friendly interface using a framework like React or Angular.  
   - Features to include:
     - Project creation form
     - Project listing page
     - Funding interface with contribution options
     - Dashboard for project creators to track funding progress

**4. Integrate Smart Contracts with Front-End**  
   - Connect the front-end application to the smart contracts on the Stellar blockchain using appropriate libraries (e.g., Stellar SDK).  
   - Ensure smooth communication between the UI and the blockchain for creating and funding projects.

**5. Test the Entire System**  
   - Conduct thorough testing of the complete application, including smart contracts and front-end integration.  
   - Perform user acceptance testing (UAT) to gather feedback and make necessary adjustments.

**6. Deployment**  
   - Deploy the smart contracts on the Stellar testnet for further testing.  
   - Launch the front-end application, ensuring proper hosting and access for users to start creating and funding projects.

## Setup Environment

Welcome to the Crowdfunding Platform! This decentralized application allows users to create and fund various projects on the Stellar Blockchain. Follow the steps below to install and run the project locally.

Table of Contents
Prerequisites
Installation
Running the Application
Usage
Contributing
License
Prerequisites
Before you begin, ensure you have the following installed:

Node.js (v14 or later)
Rust (latest stable version)
Stellar SDK (for JavaScript integration)
Yarn (optional, for package management)
Installation
Clone the Repository


git clone https://github.com/yourusername/crowdfunding-platform.git
cd crowdfunding-platform
Install Front-End Dependencies

Navigate to the front-end directory and install the required packages:


cd frontend
npm install
Install Smart Contract Dependencies

Navigate to the smart contract directory and build the contracts:


cd ../smart-contracts
cargo build --release
Running the Application
Start the Front-End Application

From the front-end directory, start the development server:


npm start
The application will be accessible at http://localhost:3000.

Deploy Smart Contracts on the Testnet

Deploy your smart contracts using the Stellar testnet. Ensure your Stellar account has sufficient test XLM to cover transaction fees.


# Command to deploy your contract
# (replace with your deployment command)
cargo run --release -- deploy
Usage
Open your browser and navigate to http://localhost:3000.
Create a new project by filling out the project creation form.
Browse existing projects and contribute funds to support them.
Monitor your projects and funding progress on your dashboard.
Contributing
We welcome contributions to improve the Crowdfunding Platform! Please follow these steps:

Fork the repository.
Create a new branch (git checkout -b feature/YourFeature).
Make your changes and commit them (git commit -m 'Add some feature').
Push to the branch (git push origin feature/YourFeature).
Open a pull request.
License
This project is licensed under the MIT License.

Thank you for your interest in the Crowdfunding Platform! We hope you find it useful. If you have any questions or issues, please feel free to open an issue in the repository.

## Programming Language
Rust & Web3 Soroban SDK  
