CREATE TABLE "database" (
  "id" int PRIMARY KEY,
  "created_time" timestamp
);

CREATE TABLE "namespace_schema" (
  "id" int PRIMARY KEY,
  "database_id" int,
  "name" varchar,
  "created_time" timestamp
);

CREATE TABLE "table_metadata" (
  "id" int PRIMARY KEY,
  "schema_id" int,
  "name" varchar,
  "created_time" timestamp
);

CREATE TABLE "table_attribute" (
  "id" int PRIMARY KEY,
  "table_id" int,
  "name" varchar,
  "base_col_number" int,
  "attnotnull" bool
);

CREATE TABLE "attribute_stats" (
  "id" int PRIMARY KEY,
  "attribute_id" int,
  "epoch_id" int,
  "name" varchar,
  "created_time" timestamp
);

CREATE TABLE "event" (
  "epoch_id" int PRIMARY KEY,
  "source_variant" varchar,
  "create_timestamp" timestamp,
  "data" json
);

ALTER TABLE "namespace_schema" ADD FOREIGN KEY ("database_id") REFERENCES "database" ("id");

ALTER TABLE "table_metadata" ADD FOREIGN KEY ("schema_id") REFERENCES "namespace_schema" ("id");

ALTER TABLE "table_attribute" ADD FOREIGN KEY ("table_id") REFERENCES "table_metadata" ("id");

ALTER TABLE "attribute_stats" ADD FOREIGN KEY ("attribute_id") REFERENCES "table_attribute" ("id");

ALTER TABLE "attribute_stats" ADD FOREIGN KEY ("epoch_id") REFERENCES "event" ("epoch_id");
