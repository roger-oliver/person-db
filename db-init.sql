
-- Enable UUID generation function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Ensure trigram search extension is enabled
-- pay attention in which schema you run it!!! all objects will be created there!!!
CREATE EXTENSION IF NOT EXISTS PG_TRGM;

-- Create table with improved structure
CREATE TABLE IF NOT EXISTS PEOPLE (
    ID UUID PRIMARY KEY,
    NICKNAME VARCHAR(32) NOT NULL,  
    FULL_NAME VARCHAR(100) NOT NULL,  
    BIRTH DATE NOT NULL,
    SKILLS TEXT[] NULL,  
    SEARCH_TRGM TEXT NOT NULL DEFAULT ''  -- ✅ Now it's a normal column
);

-- Create a function to update SEARCH_TRGM automatically
CREATE OR REPLACE FUNCTION update_search_trgm() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.SEARCH_TRGM := LOWER(NEW.FULL_NAME || ' ' || NEW.NICKNAME || ' ' || array_to_string(NEW.SKILLS, ' '));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Create a trigger to update SEARCH_TRGM before insert/update
CREATE TRIGGER people_search_trgm_trigger
BEFORE INSERT OR UPDATE ON PEOPLE
FOR EACH ROW EXECUTE FUNCTION update_search_trgm();

-- Create trigram index for fast searching
CREATE INDEX CONCURRENTLY IF NOT EXISTS IDX_PEOPLE_SEARCH_TRGM 
ON PEOPLE USING GIN (SEARCH_TRGM GIN_TRGM_OPS);
