-- Initial migration: Enable extensions and create audit functions
-- This migration sets up the foundation for all future tables

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Helper function to create audit trigger for any table
-- Usage: SELECT create_audit_trigger('table_name');
CREATE OR REPLACE FUNCTION create_audit_trigger(table_name TEXT)
RETURNS VOID AS $$
BEGIN
    EXECUTE format('
        CREATE TRIGGER update_%I_updated_at
        BEFORE UPDATE ON %I
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_column()
    ', table_name, table_name);
END;
$$ LANGUAGE plpgsql;

-- Comment documenting the audit columns pattern
COMMENT ON FUNCTION update_updated_at_column() IS
    'Automatically updates the updated_at column to NOW() on row update.
     All tables should include: created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()';
