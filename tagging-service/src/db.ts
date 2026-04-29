import { Pool } from "pg";

export const pool = new Pool({
  connectionString:
    process.env.DATABASE_URL ||
    "postgresql://postgres:postgres@localhost:5432/soroban_registry",
  max: 10,
  idleTimeoutMillis: 30000,
});
