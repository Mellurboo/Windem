const puppeteer = require('puppeteer-core');

(async () => {
  const url = process.argv[2];
  const choice = process.argv[3];

  if (!url) {
    console.error('Please provide a URL.');
    process.exit(1);
  }

  try {
    const browser = await puppeteer.launch({
      args: ['--disable-setuid-sandbox'],
      executablePath: '/snap/bin/chromium', // Replace with your actual path
      headless: true // Set to false if you want to see the browser
    });
    const page = await browser.newPage();
    await page.goto(url, { waitUntil: 'domcontentloaded' });

    if (choice == "HTML") {
      const content = await page.content();
      console.log(content);
    } else if (choice == "TITLE") {
      const title = await page.title();
      console.log(title);
    }

    await browser.close();
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
})();

