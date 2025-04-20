-- Your SQL goes here
-- Функция, обновляющая updated_at при изменении строки
-- Функция для автоматического обновления updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


DO $$ 
DECLARE 
    r RECORD;
BEGIN
    FOR r IN 
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'public'
    LOOP
        -- Проверяем, есть ли у таблицы колонка updated_at
        IF EXISTS (
            SELECT 1 FROM information_schema.columns 
            WHERE table_name = r.table_name 
            AND column_name = 'updated_at'
        ) THEN
            EXECUTE format('
                CREATE TRIGGER trigger_updated_at
                BEFORE UPDATE ON %I
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
            ', r.table_name);
        END IF;
    END LOOP;
END $$;
