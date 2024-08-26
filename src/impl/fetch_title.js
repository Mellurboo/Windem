const puppeteer = require('puppeteer-core');

(async () => {
  const url = process.argv[2];
  const choice = process.argv[3];
  const delay = parseInt(process.argv[4], 10) || 0; // Delay (in ms), only used for polling
  const pollInterval = 500; // Poll every 500 ms

  if (!url) {
    console.error('Please provide a URL.');
    process.exit(1);
  }

  try {
    const browser = await puppeteer.launch({
      args: ['--disable-setuid-sandbox'],
      executablePath: 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe', // Path to Microsoft Edge
      headless: true // Set to false if you want to see the browser
    });

    const page = await browser.newPage();
    await page.goto(url, { waitUntil: 'domcontentloaded' });

    if (choice === "HTML") {
      // Non-polling branch for HTML content
      const content = await page.content();
      console.log(content);
    } else if (choice === "TITLE") {
      // Polling branch for title changes
      let titlesString = '';
      let lastTitle = '';

      const checkTitle = async () => {
        try {
          const currentTitle = await page.title();
          if (currentTitle !== lastTitle) {
            titlesString += currentTitle + '\n';
            lastTitle = currentTitle;
          }
        } catch (error) {
          console.error('Error fetching title:', error);
        }
      };

      // Start polling for title changes
      const pollForTitleChanges = setInterval(async () => {
        await checkTitle();
      }, pollInterval);

      // Perform an initial title check
      await checkTitle();

      // Wait for the specified delay
      if (delay > 0) {
        await new Promise(resolve => setTimeout(resolve, delay));
      }

      // Stop polling and output the collected titles
      clearInterval(pollForTitleChanges);
      console.log(titlesString.trim());
    }

    await browser.close();
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
})();
