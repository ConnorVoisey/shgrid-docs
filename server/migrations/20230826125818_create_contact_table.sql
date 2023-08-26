CREATE TABLE "contact"
(
  id uuid primary key default uuid_generate_v1mc(),
  first_name text null,
  last_name text null,
  email text not null,
  mobile text not null,
  active boolean default true,
  organisation_id uuid null,

  created_at timestamptz null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('"contact"')
