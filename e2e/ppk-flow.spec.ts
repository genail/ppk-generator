import { test, expect, type Page } from '@playwright/test';

// Valid PESEL: male born 1944-05-14
const TEST_PESEL = '44051401458';
// Valid PESEL: female born 1990-03-15
// 9 0 2 3 1 5 = year 90, month 03, day 15
// digits: 9 0 0 3 1 5 _ _ _ _ _
// Let me use a known valid one: 90030515836
// Weights: 1,3,7,9,1,3,7,9,1,3
// 9*1+0*3+0*7+3*9+0*1+5*3+1*7+5*9+8*1+3*3 = 9+0+0+27+0+15+7+45+8+9 = 120
// check = (10 - 120%10)%10 = (10-0)%10 = 0. But last digit is 6. Not valid.
// Let me just use a second male PESEL: 85010112345
// Actually I'll compute: 8*1+5*3+0*7+1*9+0*1+1*3+1*7+2*9+3*1+4*3 = 8+15+0+9+0+3+7+18+3+12 = 75
// check = (10 - 75%10)%10 = (10-5)%10 = 5. So PESEL = 85010112345. Last digit should be 5. Good!
// month 01 → 1900+85=1985, month=01, day=01, digit[9]=4 even → K (female)
const TEST_PESEL_2 = '85010112345';

// --- Helpers ---

async function createOrganization(page: Page, name = 'Test Sp. z o.o.') {
  // Scope to the org selector specifically (it has the "+ Nowa organizacja" option)
  const orgSelector = page.locator('select:has(option:text("+ Nowa organizacja"))');
  await orgSelector.selectOption({ label: '+ Nowa organizacja' });

  // If an org is already selected, the form loads in edit mode — click "+ Nowa" to reset
  const editHeading = page.getByRole('heading', { name: 'Edytuj organizację' });
  if (await editHeading.isVisible()) {
    await page.getByRole('button', { name: '+ Nowa' }).click();
  }
  await expect(page.getByRole('heading', { name: 'Nowa organizacja' })).toBeVisible();

  await page.getByPlaceholder('Nazwa firmy').fill(name);
  await page.getByPlaceholder('0000000000', { exact: true }).fill('1234563218');
  await page.getByPlaceholder('000000000', { exact: true }).fill('123456785');
  await page.getByPlaceholder('Imię Nazwisko').fill('Jan Kowalski');
  await page.getByRole('button', { name: 'Utwórz' }).click();
  await expect(page.getByText('Organizacja utworzona')).toBeVisible();
}

async function addMember(page: Page, pesel: string, firstName: string, lastName: string) {
  await page.getByRole('button', { name: '👥 Uczestnicy' }).click();
  await page.getByRole('button', { name: '+ Dodaj uczestnika' }).click();
  await page.getByPlaceholder('00000000000').pressSequentially(pesel);
  await expect(page.locator('select[disabled]')).toHaveValue(/.+/);
  await page.getByRole('textbox').nth(1).fill(firstName);
  await page.getByRole('textbox').nth(2).fill(lastName);
  await page.getByRole('button', { name: 'Dodaj' }).click();
  await expect(page.getByText('Uczestnik dodany')).toBeVisible();
}

async function goToContributions(page: Page) {
  await page.getByRole('button', { name: '💰 Składki' }).click();
}

async function goToMembers(page: Page) {
  await page.getByRole('button', { name: '👥 Uczestnicy' }).click();
}

async function goToGenerations(page: Page) {
  await page.getByRole('button', { name: '📄 Generacje' }).click();
}

async function goToOrgForm(page: Page) {
  await page.getByRole('button', { name: '🏢 Organizacja' }).click();
}

async function prefillContributions(page: Page) {
  await page.getByRole('button', { name: 'Przenies z poprzedniego' }).click();
  await expect(page.getByText(/Przeniesiono składki/)).toBeVisible();
}

async function generateAndClose(page: Page) {
  await page.getByRole('button', { name: 'Generuj plik PPK' }).click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.getByRole('button', { name: 'Zamknij' }).click();
}

// --- Tests ---

test.describe('PPK Generator — mock backend', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  // =====================
  // Existing core tests
  // =====================

  test('app loads with mock backend in browser mode', async ({ page }) => {
    const logs: string[] = [];
    page.on('console', msg => logs.push(msg.text()));

    await page.goto('/');
    await expect(page.getByText('Wybierz lub utwórz organizację')).toBeVisible();

    await page.getByRole('combobox').selectOption('+ Nowa organizacja');
    await expect.poll(() => logs.some(l => l.includes('[mock]'))).toBe(true);
  });

  test('create organization and see it in selector', async ({ page }) => {
    await createOrganization(page);

    const combobox = page.getByRole('combobox').first();
    await expect(combobox).toContainText('Test Sp. z o.o.');
    await expect(page.getByText('Edytuj organizację')).toBeVisible();
  });

  test('add member with valid PESEL', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');

    await expect(page.getByText('Nowak Adam')).toBeVisible();
    await expect(page.getByText(TEST_PESEL)).toBeVisible();
    await expect(page.getByText('1944-05-14')).toBeVisible();
    await expect(page.getByText('Aktywny')).toBeVisible();
  });

  test('PESEL validation auto-fills gender and date of birth', async ({ page }) => {
    await createOrganization(page);
    await goToMembers(page);
    await page.getByRole('button', { name: '+ Dodaj uczestnika' }).click();

    await page.getByPlaceholder('00000000000').pressSequentially(TEST_PESEL);

    const genderSelect = page.locator('select[disabled]');
    await expect(genderSelect).toHaveValue('M');

    const dateInput = page.locator('input[type="date"]');
    await expect(dateInput).toHaveValue('1944-05-14');
  });

  test('prefill creates contribution rows for active members', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    await expect(page.getByText('Nowak Adam')).toBeVisible();
    const inputs = page.getByRole('textbox');
    await expect(inputs.first()).toHaveValue('0,00');
  });

  test('generate PPK with correct totals', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('94,38');
    await inputs.nth(2).fill('70,79');

    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();

    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText('94,38 zł')).toBeVisible();
    await expect(dialog.getByText('70,79 zł')).toBeVisible();
  });

  test('generate flushes pending saves (no race condition)', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('100,00');
    await inputs.nth(2).fill('50,00');

    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();

    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText('100,00 zł')).toBeVisible();
    await expect(dialog.getByText('50,00 zł')).toBeVisible();
  });

  test('generation appears in generation log', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();
    await page.getByRole('button', { name: 'Zamknij' }).click();

    await goToGenerations(page);

    await expect(page.getByText('Historia generacji')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Pobierz ZIP' })).toBeVisible();
    await expect(page.getByRole('cell', { name: '1', exact: true })).toBeVisible();
  });

  test('delete organization cascades', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');

    await goToOrgForm(page);
    await page.getByRole('button', { name: 'Usuń' }).click();

    const confirmDialog = page.getByRole('dialog');
    await expect(confirmDialog).toBeVisible();
    await confirmDialog.getByRole('button', { name: 'Usuń' }).click();

    await expect(page.getByText('Organizacja usunięta')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Nowa organizacja' })).toBeVisible();
    const combobox = page.getByRole('combobox').first();
    await expect(combobox).not.toContainText('Test Sp. z o.o.');
  });

  // =====================
  // 1. Edit member
  // =====================

  test('edit member — update name and status', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');

    // Click edit on the member row
    await page.getByRole('button', { name: 'Edytuj' }).click();
    await expect(page.getByRole('heading', { name: 'Edytuj uczestnika' })).toBeVisible();

    // PESEL should be disabled in edit mode (Svelte sets .value property, not HTML attribute)
    const peselInput = page.locator('input[type="text"][disabled]');
    await expect(peselInput).toBeVisible();
    await expect(peselInput).toHaveValue(TEST_PESEL);

    // Change name (nth(0) is the disabled PESEL input, so first/last name are nth(1)/nth(2))
    await page.getByRole('textbox').nth(1).fill('Andrzej');
    await page.getByRole('textbox').nth(2).fill('Kowalski');

    // Change status to resigned — scope to the status select (has 'resigned' option)
    await page.locator('select:has(option[value="resigned"])').selectOption('resigned');

    await page.getByRole('button', { name: 'Zapisz' }).click();
    await expect(page.getByText('Uczestnik zaktualizowany')).toBeVisible();

    // Verify changes in the member list
    await expect(page.getByText('Kowalski Andrzej')).toBeVisible();
    await expect(page.getByText('Rezygnacja')).toBeVisible();
  });

  // =====================
  // 2. Delete member
  // =====================

  test('delete member removes them from the list', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');

    // Click Usuń on the member row
    await page.getByRole('cell', { name: 'Edytuj Usuń' }).getByRole('button', { name: 'Usuń' }).click();

    // Confirm in dialog
    const confirmDialog = page.getByRole('dialog');
    await expect(confirmDialog).toBeVisible();
    await expect(confirmDialog.getByText('Adam Nowak')).toBeVisible();
    await confirmDialog.getByRole('button', { name: 'Usuń' }).click();

    await expect(page.getByText('Uczestnik usunięty')).toBeVisible();
    await expect(page.getByText('Brak uczestników')).toBeVisible();
  });

  test('delete member cascades contributions', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    // Verify contribution row exists
    await expect(page.getByText('Nowak Adam')).toBeVisible();

    // Go back to members and delete
    await goToMembers(page);
    await page.getByRole('cell', { name: 'Edytuj Usuń' }).getByRole('button', { name: 'Usuń' }).click();
    const confirmDialog = page.getByRole('dialog');
    await confirmDialog.getByRole('button', { name: 'Usuń' }).click();
    await expect(page.getByText('Uczestnik usunięty')).toBeVisible();

    // Go to contributions — table should be empty now
    await goToContributions(page);
    await expect(page.getByText('Brak składek dla wybranego okresu')).toBeVisible();
  });

  // =====================
  // 3. Edit organization
  // =====================

  test('edit organization — update name', async ({ page }) => {
    await createOrganization(page);

    // We're already on the org form after creation
    await expect(page.getByText('Edytuj organizację')).toBeVisible();

    // Change the name
    await page.getByPlaceholder('Nazwa firmy').fill('Updated Sp. z o.o.');
    await page.getByRole('button', { name: 'Zapisz' }).click();
    await expect(page.getByText('Organizacja zaktualizowana')).toBeVisible();

    // Verify selector updated
    const combobox = page.getByRole('combobox').first();
    await expect(combobox).toContainText('Updated Sp. z o.o.');
  });

  // =====================
  // 4. Upsert update path (edit existing contribution)
  // =====================

  test('editing an existing contribution updates the value', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    const inputs = page.getByRole('textbox');

    // Set initial value
    await inputs.nth(0).fill('50,00');
    // Wait for debounce save
    await page.waitForTimeout(600);

    // Now update the same field to a different value
    await inputs.nth(0).fill('75,50');
    // Wait for debounce save
    await page.waitForTimeout(600);

    // Generate and verify the updated value is used
    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText('75,50 zł')).toBeVisible();
  });

  // =====================
  // 5. Export/download ZIP
  // =====================

  test('Zapisz ZIP in generation summary triggers save', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();

    // Click Zapisz ZIP
    await dialog.getByRole('button', { name: 'Zapisz ZIP' }).click();
    await expect(page.getByText('Plik ZIP zapisany pomyślnie')).toBeVisible();
  });

  test('Pobierz ZIP in generation log triggers export', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    await generateAndClose(page);

    await goToGenerations(page);
    await page.getByRole('button', { name: 'Pobierz ZIP' }).click();
    await expect(page.getByText('Plik ZIP zapisany')).toBeVisible();
  });

  // =====================
  // 6. Organization switching / multi-org isolation
  // =====================

  test('switching organizations shows correct members', async ({ page }) => {
    // Create first org with a member
    await createOrganization(page, 'Org Alpha');
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToMembers(page);
    await expect(page.getByText('Nowak Adam')).toBeVisible();

    // Create second org with a different member
    await createOrganization(page, 'Org Beta');
    await addMember(page, TEST_PESEL_2, 'Ewa', 'Wiśniewska');
    await goToMembers(page);
    await expect(page.getByText('Wiśniewska Ewa')).toBeVisible();
    // Org Alpha's member should NOT be visible
    await expect(page.getByText('Nowak Adam')).not.toBeVisible();

    // Switch back to Org Alpha — scope to the org selector (has "+ Nowa organizacja" option)
    const orgSelector = page.locator('select:has(option:text("+ Nowa organizacja"))');
    await orgSelector.selectOption({ label: 'Org Alpha' });
    await goToMembers(page);
    await expect(page.getByText('Nowak Adam')).toBeVisible();
    await expect(page.getByText('Wiśniewska Ewa')).not.toBeVisible();
  });

  test('switching organizations shows correct contributions', async ({ page }) => {
    // Create first org, add member, prefill contributions with amounts
    await createOrganization(page, 'Org Alpha');
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);
    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('100,00');
    await inputs.nth(0).blur();
    await page.waitForTimeout(600);

    // Create second org — contributions should be empty
    await createOrganization(page, 'Org Beta');
    await goToContributions(page);
    await expect(page.getByText('Brak składek dla wybranego okresu')).toBeVisible();

    // Switch back to Org Alpha — scope to the org selector
    const orgSelector = page.locator('select:has(option:text("+ Nowa organizacja"))');
    await orgSelector.selectOption({ label: 'Org Alpha' });
    await goToContributions(page);
    await expect(page.getByText('Nowak Adam')).toBeVisible();
  });

  // =====================
  // 7. Period selector
  // =====================

  test('changing period shows different contribution data', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    // Fill amounts for January
    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('100,00');
    await page.waitForTimeout(600);

    // Switch to a different month (February)
    const monthSelect = page.locator('select').nth(1);
    await monthSelect.selectOption('2'); // Luty

    // Different period should have no contributions
    await expect(page.getByText('Brak składek dla wybranego okresu')).toBeVisible();

    // Switch back to January — data should be there
    await monthSelect.selectOption('1'); // Styczeń
    await expect(page.getByText('Nowak Adam')).toBeVisible();
  });

  // =====================
  // 8. Validation errors
  // =====================

  test('org creation fails with invalid NIP', async ({ page }) => {
    await page.getByRole('combobox').selectOption('+ Nowa organizacja');
    await page.getByPlaceholder('Nazwa firmy').fill('Test Org');
    await page.getByPlaceholder('0000000000', { exact: true }).fill('1234567890');
    await page.getByPlaceholder('000000000', { exact: true }).fill('123456785');
    await page.getByRole('button', { name: 'Utwórz' }).click();

    // Should show NIP validation error
    await expect(page.getByText('Nieprawidłowa suma kontrolna')).toBeVisible();
    // Org should NOT be created
    await expect(page.getByText('Organizacja utworzona')).not.toBeVisible();
  });

  test('org creation fails with invalid REGON', async ({ page }) => {
    await page.getByRole('combobox').selectOption('+ Nowa organizacja');
    await page.getByPlaceholder('Nazwa firmy').fill('Test Org');
    await page.getByPlaceholder('0000000000', { exact: true }).fill('1234563218');
    await page.getByPlaceholder('000000000', { exact: true }).fill('111111111');
    await page.getByRole('button', { name: 'Utwórz' }).click();

    await expect(page.getByText('Nieprawidłowa suma kontrolna')).toBeVisible();
    await expect(page.getByText('Organizacja utworzona')).not.toBeVisible();
  });

  test('org creation fails with empty name', async ({ page }) => {
    await page.getByRole('combobox').selectOption('+ Nowa organizacja');
    // Leave name empty
    await page.getByPlaceholder('0000000000', { exact: true }).fill('1234563218');
    await page.getByPlaceholder('000000000', { exact: true }).fill('123456785');
    await page.getByRole('button', { name: 'Utwórz' }).click();

    await expect(page.getByText('Nazwa jest wymagana')).toBeVisible();
  });

  test('member creation fails without name', async ({ page }) => {
    await createOrganization(page);
    await goToMembers(page);
    await page.getByRole('button', { name: '+ Dodaj uczestnika' }).click();

    await page.getByPlaceholder('00000000000').pressSequentially(TEST_PESEL);
    await expect(page.locator('select[disabled]')).toHaveValue('M');

    // Don't fill first/last name, just click add
    await page.getByRole('button', { name: 'Dodaj' }).click();
    await expect(page.getByText('Imię i nazwisko są wymagane')).toBeVisible();
  });

  test('member creation fails with invalid PESEL', async ({ page }) => {
    await createOrganization(page);
    await goToMembers(page);
    await page.getByRole('button', { name: '+ Dodaj uczestnika' }).click();

    // Type an invalid PESEL (bad checksum)
    await page.getByPlaceholder('00000000000').pressSequentially('12345678901');
    await expect(page.getByText('Nieprawidłowa suma kontrolna')).toBeVisible();

    // Fill names and try to submit
    await page.getByRole('textbox').nth(1).fill('Test');
    await page.getByRole('textbox').nth(2).fill('User');
    await page.getByRole('button', { name: 'Dodaj' }).click();

    // Should show PESEL error
    await expect(page.getByText('Wprowadź prawidłowy PESEL')).toBeVisible();
  });

  // =====================
  // 9. Prefill with previous period data
  // =====================

  test('prefill copies amounts from previous period', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);

    // Set up December 2025 as the "previous period"
    const monthSelect = page.locator('select').nth(1);
    const yearSelect = page.locator('select').nth(2);
    await monthSelect.selectOption('12');
    await yearSelect.selectOption('2025');

    // Prefill creates a zero row
    await page.getByRole('button', { name: 'Przenies z poprzedniego' }).click();
    await expect(page.getByText(/Przeniesiono składki/)).toBeVisible();

    // Set amounts for Dec 2025
    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('200,00');
    await inputs.nth(2).fill('150,00');
    // Blur the last input so MoneyInput fires onchange
    await inputs.nth(2).blur();
    await page.waitForTimeout(600);

    // Switch to January 2026
    await monthSelect.selectOption('1');
    await yearSelect.selectOption('2026');

    // Prefill from previous period (Dec 2025)
    await page.getByRole('button', { name: 'Przenies z poprzedniego' }).click();
    await expect(page.getByText(/Przeniesiono składki/)).toBeVisible();

    // Amounts should be copied from December
    const janInputs = page.getByRole('textbox');
    await expect(janInputs.nth(0)).toHaveValue('200,00');
    await expect(janInputs.nth(2)).toHaveValue('150,00');
  });

  // =====================
  // 10. Inactive member filtering
  // =====================

  test('inactive members are excluded from prefill and generation', async ({ page }) => {
    await createOrganization(page);

    // Add two members
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await addMember(page, TEST_PESEL_2, 'Ewa', 'Wiśniewska');

    // Edit the second member to resign — scope to the status select (has 'resigned' option)
    await page.getByRole('row', { name: /Wiśniewska/ }).getByRole('button', { name: 'Edytuj' }).click();
    await page.locator('select:has(option[value="resigned"])').selectOption('resigned');
    await page.getByRole('button', { name: 'Zapisz' }).click();
    await expect(page.getByText('Uczestnik zaktualizowany')).toBeVisible();

    // Go to contributions and prefill
    await goToContributions(page);
    await prefillContributions(page);

    // Only the active member (Nowak) should have a contribution row
    await expect(page.getByText('Nowak Adam')).toBeVisible();
    await expect(page.getByText('Wiśniewska Ewa')).not.toBeVisible();

    // Generate — should show 1 member, not 2
    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('100,00');
    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();

    const dialog = page.getByRole('dialog');
    await expect(dialog.getByText('100,00 zł')).toBeVisible();
    // Member count should be 1
    const memberCountLine = dialog.locator('text=Liczba uczestników:').locator('..');
    await expect(memberCountLine).toContainText('1');
  });

  // =====================
  // 11. Additional contribution fields
  // =====================

  test('all four contribution fields appear in generation totals', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('94,38');  // employee_basic
    await inputs.nth(1).fill('25,00');  // employee_additional
    await inputs.nth(2).fill('70,79');  // employer_basic
    await inputs.nth(3).fill('12,50');  // employer_additional

    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();

    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText('94,38 zł')).toBeVisible();
    await expect(dialog.getByText('25,00 zł')).toBeVisible();
    await expect(dialog.getByText('70,79 zł')).toBeVisible();
    await expect(dialog.getByText('12,50 zł')).toBeVisible();
  });

  // =====================
  // 12. Multiple generations for same period
  // =====================

  // =====================
  // 13. Reduced basic flag (T/N)
  // =====================

  test('reduced basic flag toggles and persists', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    // The flag select defaults to "N"
    const flagSelect = page.locator('select:has(option[value="T"])');
    await expect(flagSelect).toHaveValue('N');

    // Toggle to "T"
    await flagSelect.selectOption('T');
    // Wait for debounce save
    await page.waitForTimeout(600);

    // Switch to a different month and back to verify persistence
    const monthSelect = page.locator('select').nth(1);
    await monthSelect.selectOption('2');
    await expect(page.getByText('Brak składek dla wybranego okresu')).toBeVisible();

    await monthSelect.selectOption('1');
    await expect(page.getByText('Nowak Adam')).toBeVisible();

    // Flag should still be "T"
    await expect(flagSelect).toHaveValue('T');
  });

  // =====================
  // 14. Multiple members — totals sum correctly
  // =====================

  test('contribution totals sum across multiple members', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await addMember(page, TEST_PESEL_2, 'Ewa', 'Wiśniewska');
    await goToContributions(page);
    await prefillContributions(page);

    // Two member rows should be visible
    await expect(page.getByText('Nowak Adam')).toBeVisible();
    await expect(page.getByText('Wiśniewska Ewa')).toBeVisible();

    // Fill amounts for member 1 (Nowak): 4 inputs per row
    const inputs = page.getByRole('textbox');
    await inputs.nth(0).fill('100,00');  // Nowak employee_basic
    await inputs.nth(1).fill('10,00');   // Nowak employee_additional
    await inputs.nth(2).fill('75,00');   // Nowak employer_basic
    await inputs.nth(3).fill('5,00');    // Nowak employer_additional

    // Fill amounts for member 2 (Wiśniewska)
    await inputs.nth(4).fill('200,00');  // Wiśniewska employee_basic
    await inputs.nth(5).fill('20,00');   // Wiśniewska employee_additional
    await inputs.nth(6).fill('150,00');  // Wiśniewska employer_basic
    await inputs.nth(7).fill('15,00');   // Wiśniewska employer_additional

    // Blur last input to trigger save
    await inputs.nth(7).blur();

    // Verify footer totals: 100+200=300, 10+20=30, 75+150=225, 5+15=20
    const footer = page.locator('tfoot');
    await expect(footer).toContainText('300,00');
    await expect(footer).toContainText('30,00');
    await expect(footer).toContainText('225,00');
    await expect(footer).toContainText('20,00');

    // Generate and verify dialog totals + member count
    await page.getByRole('button', { name: 'Generuj plik PPK' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText('300,00 zł')).toBeVisible();
    await expect(dialog.getByText('30,00 zł')).toBeVisible();
    await expect(dialog.getByText('225,00 zł')).toBeVisible();
    await expect(dialog.getByText('20,00 zł')).toBeVisible();
    // Member count should be 2
    await expect(dialog.getByText('2', { exact: true })).toBeVisible();
  });

  // =====================
  // 15. Multiple generations for same period
  // =====================

  test('multiple generations for same period appear in log', async ({ page }) => {
    await createOrganization(page);
    await addMember(page, TEST_PESEL, 'Adam', 'Nowak');
    await goToContributions(page);
    await prefillContributions(page);

    // Generate twice
    await generateAndClose(page);
    await generateAndClose(page);

    // Check generation log
    await goToGenerations(page);

    // Should have 2 "Pobierz ZIP" buttons
    const exportButtons = page.getByRole('button', { name: 'Pobierz ZIP' });
    await expect(exportButtons).toHaveCount(2);
  });
});
