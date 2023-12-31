CREATE TABLE i18n_text (
    id            UUID        PRIMARY KEY,
    default_text  TEXT        NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL,
    update_date   TIMESTAMPTZ NOT NULL
);

CREATE TABLE language (
    id                UUID        PRIMARY KEY,
    code              TEXT        NOT NULL
                                  UNIQUE,
    original_name     TEXT        NOT NULL,
    i18n_text_id_name UUID        NOT NULL
                                  REFERENCES i18n_text,
    website_title     TEXT        NOT NULL,
    website_subtitle  TEXT        NOT NULL,
    creation_date     TIMESTAMPTZ NOT NULL,
    update_date       TIMESTAMPTZ NOT NULL
);

CREATE TABLE i18n_translation (
    id            UUID        PRIMARY KEY,
    i18n_text_id  UUID        NOT NULL
                              REFERENCES i18n_text ON DELETE CASCADE,
    language_id   UUID        NOT NULL
                              REFERENCES language ON DELETE CASCADE,
    text          TEXT        NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL,
    update_date   TIMESTAMPTZ NOT NULL,

    UNIQUE (i18n_text_id, language_id)
);

INSERT INTO i18n_text (id, default_text, creation_date, update_date)
VALUES ('3347bde2-aca4-4466-b989-941cde85799f'::UUID, 'English', NOW(), NOW());

INSERT INTO i18n_text (id, default_text, creation_date, update_date)
VALUES ('5dcb5e3e-862c-4dd5-a13a-fded43ec146e'::UUID, 'Español', NOW(), NOW());

INSERT INTO language (
    id,
    code,
    original_name,
    i18n_text_id_name,
    website_title,
    website_subtitle,
    creation_date,
    update_date
) VALUES (
    'f574ada6-88e6-464b-a9ed-16c83ba6f900'::UUID,
    'en',
    'English',
    '3347bde2-aca4-4466-b989-941cde85799f'::UUID,
    'Example',
    'Made with Tukosmo',
    NOW(),
    NOW()
);

INSERT INTO language (
    id,
    code,
    original_name,
    i18n_text_id_name,
    website_title,
    website_subtitle,
    creation_date,
    update_date
) VALUES (
    'd24c014a-e58c-4fff-8f15-be4b69385a97'::UUID,
    'es',
    'Español',
    '5dcb5e3e-862c-4dd5-a13a-fded43ec146e'::UUID,
    'Ejemplo',
    'Hecho con Tukosmo',
    NOW(),
    NOW()
);

INSERT INTO i18n_translation (
    id,
    i18n_text_id,
    language_id,
    text,
    creation_date,
    update_date
)
VALUES (
    '09a6048a-cd48-44bf-a3d3-8dc74be32c28'::UUID,
    '3347bde2-aca4-4466-b989-941cde85799f'::UUID,
    'f574ada6-88e6-464b-a9ed-16c83ba6f900'::UUID,
    'English',
    NOW(),
    NOW()
);

INSERT INTO i18n_translation (
    id,
    i18n_text_id,
    language_id,
    text,
    creation_date,
    update_date
)
VALUES (
    '24931540-51f7-45f7-ac63-a1b3c2b397b8'::UUID,
    '3347bde2-aca4-4466-b989-941cde85799f'::UUID,
    'd24c014a-e58c-4fff-8f15-be4b69385a97'::UUID,
    'Inglés',
    NOW(),
    NOW()
);

INSERT INTO i18n_translation (
    id,
    i18n_text_id,
    language_id,
    text,
    creation_date,
    update_date
)
VALUES (
    'b7667652-346a-4090-a35c-ddf0f04d687d'::UUID,
    '5dcb5e3e-862c-4dd5-a13a-fded43ec146e'::UUID,
    'f574ada6-88e6-464b-a9ed-16c83ba6f900'::UUID,
    'Spanish',
    NOW(),
    NOW()
);

INSERT INTO i18n_translation (
    id,
    i18n_text_id,
    language_id,
    text,
    creation_date,
    update_date
)
VALUES (
    'be55d6ab-d5aa-4a32-9e81-d02f504ff6be'::UUID,
    '5dcb5e3e-862c-4dd5-a13a-fded43ec146e'::UUID,
    'd24c014a-e58c-4fff-8f15-be4b69385a97'::UUID,
    'Español',
    NOW(),
    NOW()
);
