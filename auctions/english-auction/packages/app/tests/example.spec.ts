import { test, expect } from '@playwright/test';

test.describe("e2e", () => {
  test('Seller creates auction', async ({ page }) => {
    await page.goto('localhost:3000/sell');
    
  });
});
