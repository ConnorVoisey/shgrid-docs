CREATE TABLE "organisation"
(
  id uuid primary key default uuid_generate_v1mc(),
  name text null,
  postcode text null,
  active boolean default true,

  created_at timestamptz null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('"organisation"');

ALTER TABLE contact
    ADD CONSTRAINT fk_contact_organisation FOREIGN KEY (organisation_id) REFERENCES organisation (id);
