DO $$ 
DECLARE 
    r RECORD;
BEGIN
    FOR r IN 
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'public'
    LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS trigger_updated_at ON %I;', r.table_name);
    END LOOP;
END $$;

DROP FUNCTION IF EXISTS update_updated_at_column();
