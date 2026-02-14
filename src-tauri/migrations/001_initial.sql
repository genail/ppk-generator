CREATE TABLE organizations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    nip TEXT NOT NULL,
    regon TEXT NOT NULL,
    contact_person TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    pesel TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    gender TEXT NOT NULL CHECK(gender IN ('M', 'K')),
    date_of_birth TEXT NOT NULL,
    citizenship TEXT NOT NULL DEFAULT 'PL',
    second_name TEXT NOT NULL DEFAULT '',
    doc_type TEXT NOT NULL DEFAULT '',
    doc_number TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'resigned', 'terminated')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(organization_id, pesel)
);

CREATE TABLE contributions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    member_id INTEGER NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    period_year INTEGER NOT NULL,
    period_month INTEGER NOT NULL CHECK(period_month >= 1 AND period_month <= 12),
    employee_basic TEXT NOT NULL DEFAULT '0.00',
    employee_additional TEXT NOT NULL DEFAULT '0.00',
    employer_basic TEXT NOT NULL DEFAULT '0.00',
    employer_additional TEXT NOT NULL DEFAULT '0.00',
    reduced_basic_flag TEXT NOT NULL DEFAULT 'N' CHECK(reduced_basic_flag IN ('T', 'N')),
    source TEXT NOT NULL DEFAULT 'manual' CHECK(source IN ('manual', 'prefilled')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(member_id, period_year, period_month)
);

CREATE TABLE generations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    period_year INTEGER NOT NULL,
    period_month INTEGER NOT NULL,
    generated_at TEXT NOT NULL DEFAULT (datetime('now')),
    snapshot_json TEXT NOT NULL,
    file_path TEXT NOT NULL DEFAULT '',
    total_employee_basic TEXT NOT NULL DEFAULT '0.00',
    total_employer_basic TEXT NOT NULL DEFAULT '0.00',
    member_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_members_org ON members(organization_id);
CREATE INDEX idx_contributions_member ON contributions(member_id);
CREATE INDEX idx_contributions_period ON contributions(period_year, period_month);
CREATE INDEX idx_generations_org ON generations(organization_id);
