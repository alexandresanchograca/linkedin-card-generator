# linkedin-card-generator

The objective of this application is to scrape a LinkedIn profile and provide a printable business. It creates a bussiness card with all the information present in the linkedin profile aswell as a QR code that direct's you to the person's profile page.

## Backend:

A REST API that connects to Chromedriver and opens a new browser instance, allowing it to log in to LinkedIn and scrape a user profile, returning the collected in a JSON format.

It supports two requests:

- Login to linked-in in the current browser instance
- Get profile info
(Check the web-app for request examples)

Created instances will persist for 60 minutes, after that they will be closed. If you wish to modify the way that browser instances are managed please do so in `sessionmanager.rs`

A JWT token will be generated for each instance, this token should be sent in every API request in order to identify the target instance.

## Frontend:

A React web application that sends requests to the backend API and builds a business card with the returned data. This card can be easily printed by right-clicking the current page and using your browser's built-in print functionality. The page elements will be automatically hidden when you do so. 

#### Dependencies:

**ChromeDriver**: 
You need to have a running instance of Chromedriver to run the scraper.

You can find the latest release on:
https://googlechromelabs.github.io/chrome-for-testing/

Ensure your chromedriver is compatible with the current version of your Chrome browser.
There may be issues with different versions other than the one that was used during testing.

**Node.js**:
You need to have Node installed in order to build and run the web-app.

### Building and Running:
**API**: 
In order to build the API you have to have Rust installed, I suggest you use [rustup](https://rustup.rs/) as it eases the process of installation.
Go to the root folder of the api and execute: ```cargo run``` in order to build and run the API.

**Web-app**:
To build the web-app is required to have Node installed. Go to the root directory of the web-app and install it's dependencies with `npm install` after the installation you can run the web-app with `npm run dev`. The web-app was built using the [Vite](https://vitejs.dev/).
